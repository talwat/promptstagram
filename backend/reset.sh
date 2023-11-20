#!/bin/sh

sqlx database reset
sqlx migrate run