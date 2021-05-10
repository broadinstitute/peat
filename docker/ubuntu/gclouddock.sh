#!/usr/bin/env bash
project="nitrogenase-docker"
name="peat-ubuntu"
tag="1.0.0"
full="${name}:${tag}"
echo "Using Google project ${project}, Docker project ${name}, full tag ${full}"
echo "Cloud-building Docker image:"
gcloud builds submit --timeout=60m --tag gcr.io/${project}/${full}
echo "Done"
