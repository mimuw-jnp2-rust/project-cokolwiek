#!/bin/sh

echo "Downloading libstt from https://github.com/coqui-ai/STT/releases/tag/v1.3.0"
wget https://github.com/coqui-ai/STT/releases/download/v1.3.0/libstt.tflite.Linux.zip

unzip libstt.tflite.Linux.zip
