#!/bin/bash

set -u

wsm='./gherkin2json.wasm'

run_wasi(){
  cat /dev/stdin |
  wasmtime \
    run \
    "${wsm}"
}

input1(){
  echo 'Feature: User Login'
  echo ''
  echo '  As a registered user'
  echo '  I want to be able to log into my account'
  echo '  So that I can access my private dashboard'
  echo ''
  echo '  Scenario: Successful login'
  echo '    Given the user is on the login page'
  echo '    When  the user enters a valid username "john_doe"'
  echo '    And   the user enters a valid password "passw0rd"'
  echo '    And   the user clicks the "Login" button"'
  echo '    Then  they should be redirected to the dashboard'
  echo ''
  echo '  Scenario Outline: eating <eat> eggs'
  echo '    Given there are <start> eggs'
  echo '    When  I eat <eat> eggs'
  echo '    Then  I should have <left> eggs'
  echo ''
  echo '    Examples:'
  echo '    | start | eat | left |'
  echo '    |    12 |   2 |   10 |'
  echo '    |     4 |   1 |     3|'
}

echo feature to json
input1 | run_wasi | jq -c
echo

echo feature to yaml
input1 |
  run_wasi |
  jq |
  dasel --in=json --out=yaml |
  bat --language=yaml
