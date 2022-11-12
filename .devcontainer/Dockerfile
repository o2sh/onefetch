# NOTE: see https://hub.docker.com/_/microsoft-vscode-devcontainers
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}
RUN sudo apt update && sudo apt install cmake
