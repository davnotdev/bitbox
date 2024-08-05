use anyhow::Result;
use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};
use thiserror::Error;

mod host;
mod store;
