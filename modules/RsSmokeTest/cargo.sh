#!/usr/bin/env sh


EVEREST_CORE=$HOME/checkout/everest-workspace/everest-core/

export EV_MODULE=SmokeTest
export EV_PREFIX=$EVEREST_CORE/build/dist
export EV_CONF_FILE=$HOME/Desktop/Programming/qwello/everest/everest-core/config/config-sil.yaml
export EV_DONT_VALIDATE_SCHEMA=

export LD_LIBRARY_PATH=$EVEREST_CORE/build/dist/lib

cargo $*
