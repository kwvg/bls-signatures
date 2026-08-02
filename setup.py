#!/usr/bin/env python3
# coding: latin-1

#
# Copyright (c) 2020-present, Chia Network Inc.
# Copyright (c) 2026-present, The Dash Core developers
# SPDX-License-Identifier: Apache-2.0
# See the accompanying file LICENSE or https://opensource.org/licenses/Apache-2.0
#

"""Build configuration for the dashbls package."""

import os
import platform
import re
import subprocess
import sys
import sysconfig

from setuptools import Extension, setup
from setuptools.command.build_ext import build_ext


class CMakeExtension(Extension):
    def __init__(self, name, sourcedir=""):
        super().__init__(name, sources=[])
        self.sourcedir = os.path.abspath(sourcedir)


class CMakeBuild(build_ext):
    CMAKE_MINIMUM = (3, 18, 0)

    def run(self):
        try:
            out = subprocess.check_output(["cmake", "--version"]).decode()
        except OSError:
            raise RuntimeError("CMake must be installed to build dashbls")
        found = re.search(r"version\s*([\d.]+)", out)
        if found is None:
            raise RuntimeError("cannot read a version out of: " + out.strip())
        parts = [int(part) for part in found.group(1).split(".")[:3] if part]
        while len(parts) < len(self.CMAKE_MINIMUM):
            parts.append(0)
        version = tuple(parts)
        if version < self.CMAKE_MINIMUM:
            raise RuntimeError(
                "CMake >= {} is required, found {}".format(
                    ".".join(str(p) for p in self.CMAKE_MINIMUM), found.group(1)
                )
            )
        for ext in self.extensions:
            self.build_extension(ext)

    def build_extension(self, ext):
        extdir = os.path.abspath(os.path.dirname(self.get_ext_fullpath(ext.name)))
        cmake_args = [
            "-DCMAKE_LIBRARY_OUTPUT_DIRECTORY=" + extdir,
            "-DMULTI=",
            "-DPYTHON_EXECUTABLE=" + sys.executable,
            "-DPYTHON_EXTENSION_SUFFIX=" + (sysconfig.get_config_var("EXT_SUFFIX") or ""),
        ]

        cfg = "Debug" if self.debug else "Release"
        build_args = ["--config", cfg]

        if platform.system() == "Windows":
            cmake_args += [
                "-DCMAKE_LIBRARY_OUTPUT_DIRECTORY_{}={}".format(cfg.upper(), extdir)
            ]
            if sys.maxsize > 2 ** 32:
                cmake_args += ["-A", "x64"]
            build_args += ["--", "/m"]
        else:
            cmake_args += ["-DCMAKE_BUILD_TYPE=" + cfg]
            build_args += ["--", "-j", "6"]

        env = os.environ.copy()
        env["CXXFLAGS"] = '{} -DVERSION_INFO=\\"{}\\"'.format(
            env.get("CXXFLAGS", ""), self.distribution.get_version()
        )
        if not os.path.exists(self.build_temp):
            os.makedirs(self.build_temp)
        subprocess.check_call(
            ["cmake", ext.sourcedir] + cmake_args, cwd=self.build_temp, env=env
        )
        subprocess.check_call(
            ["cmake", "--build", "."] + build_args, cwd=self.build_temp
        )


setup(
    ext_modules=[CMakeExtension("dashbls", ".")],
    cmdclass=dict(build_ext=CMakeBuild),
    zip_safe=False,
)
