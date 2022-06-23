# -*- coding: utf-8 -*-
import asyncio
import dataclasses
import multiprocessing
import pathlib
import shutil
import subprocess
import tempfile
import time
from typing import Union, Iterable, Dict, Any

import toml

LICENSE = "BSD-3-Clause"
VERSION = "0.0.1"
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
            "name": svd_descr.name,
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


async def generate_svd2rust_crate(svd_descr: SvdMeta, pac_family: str) -> None:
    print(svd_descr.name, svd_descr.path)

    async with CRATE_GEN_SEM:
        with tempfile.TemporaryDirectory() as tmpd:
            subprocess.run(
                ["svd2rust", "-i", f"{svd_descr.path}", "-o", f"{tmpd}"],
                check=True,
            )
            lib_rs = pathlib.Path(tmpd, "lib.rs")
            subprocess.run(
                [
                    "form",
                    "-i",
                    f"{lib_rs}",
                    "-o",
                    f"{tmpd}/src",
                ],
                check=True,
            )
            lib_rs.unlink()
            out_dir = pathlib.Path("pacs", f"{pac_family}", f"{svd_descr.name}")

            if not out_dir.exists():
                out_dir.mkdir(parents=True)

            for element in pathlib.Path(tmpd).iterdir():
                shutil.move(element, out_dir)

            with out_dir.joinpath("Cargo.toml").open("w+") as cargo_toml:
                content = mcu_crate_toml_template(svd_descr)
                toml.dump(content, cargo_toml)

            subprocess.run(["cargo", "fmt"], cwd=out_dir, check=True)
            subprocess.run(["rustfmt", "build.rs"], cwd=out_dir, check=True)


def walk_svd_files(svd_dir: Union[str, pathlib.Path]) -> Iterable[SvdMeta]:
    for svd_file in svd_dir.iterdir():
        if svd_file.suffix.endswith("svd"):
            yield SvdMeta(name=svd_file.stem.lower(), path=svd_file.resolve())


async def generate_svd2rust_crates(svd_dir: Union[str, pathlib.Path]) -> None:
    tasks = []

    for p in pathlib.Path(svd_dir).iterdir():
        if p.is_dir():
            pac_family = p.name.lower()

            for svd_file in walk_svd_files(p):
                tasks.append(
                    asyncio.create_task(generate_svd2rust_crate(svd_file, pac_family))
                )

    print("await for generation:", len(tasks))
    time.sleep(10)
    await asyncio.gather(*tasks)


def main() -> None:
    asyncio.run(generate_svd2rust_crates(SVD_DIR))


if __name__ == "__main__":
    main()
