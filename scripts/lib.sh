#!/bin/bash
export SCRIPT_DIR="$(realpath -s "$(dirname "${BASH_SOURCE[0]}")")"
export BASE_DIR="$(dirname "${SCRIPT_DIR}")"
export TMP_DIR="${BASE_DIR}/tmp"
export TMP_TESTS_DIR="${TMP_DIR}/tests"
export TMP_REPO_TEST_DIR="${TMP_TESTS_DIR}/repo"
export SCRIPT_DIR BASE_DIR

rmDirContents() {
    local path="${1}"
    if [[
        "${path}" == ""
    ]]; then
        printf "Error: Empty path specified. Can't delete the contents of "
        printf "an empty path.\n"
        return 1
    elif [[
        "${path}" == "/"
        || "${path}" == "//"
    ]]; then
        printf "Error: Not going to delete the contents of /. "
        printf "Path we almost tried to delete the contents of: %s\n" "${path}"
        return 1
    elif [[
           "${path}" == "."
    ]]; then
        printf "Error: Not going to delete the contents of \".\". "
        printf "Path we almost tried to delete the contents of: "
        printf "%s\n" "$(realpath "${path}")"
        return 1
        elif [[
           "${path}" == ".."
    ]]; then
        printf "Error: Not going to delete the contents of \"..\". "
        printf "Path we almost tried to delete the contents of: "
        printf "%s\n" "$(realpath "${path}")"
        return 1
    elif [[
           "$(realpath "${path}")" == "${SCRIPT_DIR}"
    ]]; then
        printf "Error: Looks like we almost tried to delete the scripts directory. "
        printf "Not going to do that. "
        printf "Path we almost tried to delete the contents of: %s\n" "${path}"
        return 1
    elif [[
           -d "${path}/filecastalogue"
        && -d "${path}/scripts"
        && -e "${path}/scripts/lib.sh"
    ]]; then
        printf "Error: Looks like we almost tried to delete the repository. "
        printf "Not going to do that. "
        printf "Path we almost tried to delete the contents of: %s\n" "${path}"
        return 1
    elif [[
           "$(realpath "${path}")" == "$HOME"
        || "$(realpath "${path}")" == "$USERPROFILE" # Windows
        || -f "$(realpath "${path}")/.profile" # Last, feeble defense, just in case.
    ]]; then
        printf "Looks like we almost tried to delete the contents of the home "
        printf "directory. "
        printf "Path we almost tried to delete the contents of: %s\n" "${path}"
        return 1
    fi
    printf "Deleting everything in %s ...\n" "${path:?}"
    rm -Rf "${path:?}"/*
}

rmDir() {
    local path="${1}"
    # This isn't "${path:?}" for only one reason: So we can benefit
    # from the empty path response of rmDirContents.
    rmDirContents "${path}" && rmdir "${path}"
}