#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
    echo "usage: $0 <version>  (e.g. 0.1.0)" >&2
    exit 1
fi

TAG="go/v${VERSION}"

if git rev-parse "$TAG" &>/dev/null; then
    echo "error: tag $TAG already exists" >&2
    exit 1
fi

git tag "$TAG"
git push origin "$TAG"

echo "published Go module as $TAG"
