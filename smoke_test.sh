#!/usr/bin/env bash

set -euo pipefail

HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-18080}"
BASE_URL="http://${HOST}:${PORT}"

SERVER_PID=""

cleanup() {
  if [[ -n "${SERVER_PID}" ]] && kill -0 "${SERVER_PID}" 2>/dev/null; then
    kill "${SERVER_PID}" 2>/dev/null || true
    wait "${SERVER_PID}" 2>/dev/null || true
  fi
}

trap cleanup EXIT

echo "Starting rusic server on ${HOST}:${PORT} for smoke tests..."
RUSIC_ADDR="${HOST}:${PORT}" go run . >/tmp/rusic-smoke.log 2>&1 &
SERVER_PID=$!

for _ in {1..50}; do
  if curl -sS "${BASE_URL}/" >/dev/null 2>&1; then
    break
  fi
  sleep 0.2
done

if ! curl -sS "${BASE_URL}/" >/dev/null 2>&1; then
  echo "Server did not become ready. Log output:"
  cat /tmp/rusic-smoke.log
  exit 1
fi

failures=0

check_route() {
  local path="$1"
  local code
  code=$(curl -s -o /dev/null -w "%{http_code}" "${BASE_URL}${path}" || true)
  if [[ -z "${code}" || "${code}" == "000" ]]; then
    echo "FAIL ${path} -> request_failed"
    failures=$((failures + 1))
    return
  fi
  if [[ "${code}" == "404" || "${code}" == "405" ]]; then
    echo "FAIL ${path} -> ${code}"
    failures=$((failures + 1))
    return
  fi
  echo "PASS ${path} -> ${code}"
}

check_route "/"
check_route "/main"
check_route "/randomart"
check_route "/albumofinterest"
check_route "/songsforalbum/1"
check_route "/artiststartswith"
check_route "/albumstartswith"
check_route "/currentPlayingImg/1"
check_route "/artistforalpha/a"
check_route "/albumforalpha/a"
check_route "/albumsforartist/1"
check_route "/albumsforartistsongs/1"
check_route "/songpages"
check_route "/songsforpage/1"
check_route "/playlistcheck"
check_route "/createemptyplaylist/smoke"
check_route "/createrandomplaylist/smoke/3"
check_route "/allplaylists"
check_route "/editplaylistpage/1"
check_route "/addsongtoplaylist/1/1"
check_route "/removesongfromplaylist/1/1"
check_route "/deleteplaylist/1"
check_route "/coverartfromplaypath/example"
check_route "/playmusic/1"
check_route "/playplaylist/1"

if [[ ${failures} -gt 0 ]]; then
  echo "Smoke test failed with ${failures} route errors."
  exit 1
fi

echo "Smoke test completed successfully."