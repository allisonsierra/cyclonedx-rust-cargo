/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (c) OWASP Foundation. All Rights Reserved.
 */

/**
* A special acknowledgement Ossi Herrala from SensorFu for providing a
* starting point in which to develop this plugin. The original project
* this plugin was derived from is sensorfu/cargo-bom v0.3.1 (MIT licensed)
* https://github.com/sensorfu/cargo-bom
*
* MIT License
*
* Copyright (c) 2017-2019 SensorFu Oy
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use cargo::core::Workspace;
use cargo::Config;
use cargo_cyclonedx::generator::SbomGenerator;
use std::{
    io::{self},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

#[deny(clippy::all)]
#[deny(warnings)]
mod cli;
use cli::{Args, Opts};

fn main() -> anyhow::Result<()> {
    let Opts::Bom(args) = Opts::parse();
    let mut config = Config::default()?;
    setup_logging(&args, &mut config)?;

    let manifest_path = locate_manifest(&args)?;
    let cli_config = args.as_config()?;

    let ws = Workspace::new(&manifest_path, &config)?;

    log::trace!("SBOM generation started");
    let boms = SbomGenerator::create_sboms(ws, &cli_config)?;
    log::trace!("SBOM generation finished");

    log::trace!("SBOM output started");
    for bom in boms {
        bom.write_to_file()?;
    }
    log::trace!("SBOM output finished");

    Ok(())
}

fn setup_logging(args: &Args, config: &mut Config) -> anyhow::Result<()> {
    let mut builder = Builder::new();

    // default cargo internals to quiet unless overridden via an environment variable
    // call with RUST_LOG='cargo::=debug' to access these logs
    builder.filter_module("cargo::", LevelFilter::Error);

    let level_filter = if args.quiet {
        LevelFilter::Off
    } else {
        match args.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    builder.filter_level(level_filter);

    builder.parse_default_env(); // allow overriding CLI arguments
    builder.try_init()?;

    // configure logging level of cargo to match what was passed via CLI
    config.configure(
        args.verbose as u32,
        args.quiet,
        None,
        false,
        false,
        false,
        &None,
        &[],
        &[],
    )?;

    Ok(())
}

fn locate_manifest(args: &Args) -> Result<PathBuf, io::Error> {
    if let Some(manifest_path) = &args.manifest_path {
        let manifest_path = manifest_path.canonicalize()?;
        log::info!(
            "Using manually specified Cargo.toml manifest located at: {}",
            manifest_path.to_string_lossy()
        );
        Ok(manifest_path)
    } else {
        let manifest_path = std::env::current_dir()?.join("Cargo.toml");
        log::info!(
            "Using Cargo.toml manifest located at: {}",
            manifest_path.to_string_lossy()
        );
        Ok(manifest_path)
    }
}
