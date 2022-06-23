# -*- coding: utf-8 -*-
import argparse
import asyncio
import dataclasses
import multiprocessing
import pathlib
import re
import shutil
import tempfile
from typing import Union, Iterable, Dict, Any

import toml

LICENSE = "BSD-3-Clause"
VERSION = "0.1.0"
REPOSITORY = "https://github.com/vpetrigo/efm32-pacs"
AUTHORS = ["Vladimir Petrigo <vladimir.petrigo@gmail.com>"]
SVD_DIR = pathlib.Path(".", "svd").resolve()

CRATE_GEN_SEM = asyncio.Semaphore(multiprocessing.cpu_count())


@dataclasses.dataclass
class SvdMeta:
    name: str
    path: pathlib.Path


def mcu_crate_toml_template(svd_descr: SvdMeta) -> Dict[str, Any]:
    cargo_toml_mcu_template = {
        "package": {
            "name": f"{svd_descr.name}-pac",
            "description": f"Peripheral access API for {svd_descr.name.upper()} MCU (generated using svd2rust)",
            "version": VERSION,
            "authors": AUTHORS,
            "license": LICENSE,
            "keywords": ["no-std", "arm", "cortex-m", "efm32"],
            "categories": ["embedded", "hardware-support", "no-std"],
            "repository": REPOSITORY,
            "readme": "README.md",
            "edition": "2021",
        },
        "dependencies": {
            "cortex-m": "~0.7",
            "vcell": "~0.1",
            "cortex-m-rt": {"version": "~0.7", "optional": True},
        },
        "features": {"rt": ["cortex-m-rt/device"]},
    }

    return cargo_toml_mcu_template


def pac_readme_template(svd_descr: SvdMeta) -> str:
    readme_template = rf"""# {svd_descr.name.upper()}

A peripheral access crate for the {svd_descr.name.upper()} from Silabs for Rust Embedded projects.

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.
"""

    return readme_template


async def generate_svd2rust_crate(svd_descr: SvdMeta, pac_family: str) -> None:
    print(svd_descr.name, svd_descr.path)

    async with CRATE_GEN_SEM:
        with tempfile.TemporaryDirectory() as tmpd:
            pret = await asyncio.create_subprocess_exec(
                *["svd2rust", "-i", f"{svd_descr.path}", "-o", f"{tmpd}"]
            )
            await pret.wait()
            assert pret.returncode == 0
            lib_rs = pathlib.Path(tmpd, "lib.rs")
            pret = await asyncio.create_subprocess_exec(
                *[
                    "form",
                    "-i",
                    f"{lib_rs}",
                    "-o",
                    f"{tmpd}/src",
                ],
            )

            await pret.wait()
            assert pret.returncode == 0
            lib_rs.unlink()
            out_dir = pathlib.Path("pacs", f"{pac_family}", f"{svd_descr.name}")

            if not out_dir.exists():
                out_dir.mkdir(parents=True)

            for element in pathlib.Path(tmpd).iterdir():
                shutil.move(element, out_dir)

            with out_dir.joinpath("Cargo.toml").open("w+") as cargo_toml:
                content = mcu_crate_toml_template(svd_descr)
                toml.dump(content, cargo_toml)

            pret = await asyncio.create_subprocess_exec(*["cargo", "fmt"], cwd=out_dir)
            await pret.wait()
            assert pret.returncode == 0
            pret = await asyncio.create_subprocess_exec(
                *["rustfmt", "build.rs"], cwd=out_dir
            )
            await pret.wait()
            assert pret.returncode == 0

            with out_dir.joinpath("README.md").open("w+") as readme:
                readme.write(pac_readme_template(svd_descr))


def walk_svd_files(svd_dir: Union[str, pathlib.Path]) -> Iterable[SvdMeta]:
    for svd_file in svd_dir.iterdir():
        if svd_file.suffix.endswith("svd"):
            yield SvdMeta(
                name=re.sub(r"f\d+.*$", "", svd_file.stem.lower()),
                path=svd_file.resolve(),
            )


async def generate_svd2rust_crates(svd_dir: Union[str, pathlib.Path]) -> None:
    tasks = []

    for p in pathlib.Path(svd_dir).iterdir():
        if p.is_dir():
            pac_family = p.name.lower()

            for svd_file in walk_svd_files(p):
                tasks.append(
                    asyncio.create_task(generate_svd2rust_crate(svd_file, pac_family))
                )

    await asyncio.gather(*tasks)


def main() -> None:
    parser = argparse.ArgumentParser(prog="EFM32 Helper Tooling")
    pacs_parser = parser.add_subparsers(help="Generate PACs", dest="command")
    pacs_parser.add_parser("pacs-generate", help="Run PACs generation")

    args = parser.parse_args()

    if args.command == "pacs-generate":
        asyncio.run(generate_svd2rust_crates(SVD_DIR))


if __name__ == "__main__":
    main()
