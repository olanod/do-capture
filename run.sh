#!/bin/sh
firefox -headless -marionette & 
FIREFOX_PID=$!
capture "$@"

kill $FIREFOX_PID
