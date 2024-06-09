!#/bin/bash

echo "view dev k8s"
kubectl kustomize overlays/dev

echo "view prod k8s"
kubectl kustomize overlays/prod

echo "apply dev k8s"
kubectl apply -k overlays/dev
