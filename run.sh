#!/usr/bin/env bash

cargo fmt

cargo run -- ts-files/hello-world.ts -o gen/hello-world.swift
