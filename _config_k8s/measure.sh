#!/bin/bash

for node in `seq 1 12`; do
    for loop in `seq 1 3`; do
        echo "[INFO] Verifying ${node}-${loop}."
        kubectl apply -f app-server.yml 1>/dev/null 2>&1 && sleep 20 && kubectl apply -f app-job.yml 1>/dev/null 2>&1
        while true
        do
            sleep 10
            result=`kubectl get pods|awk '{if ( $3 == "ContainerCreating" || $3 == "Running" ){print $3}}'|wc -l|ggrep -oP "\d"`
            if [ $result -eq 0 ];
            then
                break;
            fi
        done
        result=`kubectl get pods|awk '{if ( $3 == "Completed" ){print $3}}'|wc -l|ggrep -oP "\d"`
        expect_comps=$((node+1))
        if [ $result -ne $expect_comps ];
        then
            echo "[ERROR] pod status error" >&2
            exit 1
        fi
        kubectl get pods | ggrep -oP "indexed-job-\d+\S+"|xargs -I {} bash -c "kubectl logs {}"|sort > ../target/logs/${node}-${loop}.log
        kubectl delete -f app-server.yml 1>/dev/null 2>&1 && kubectl delete -f app-job.yml 1>/dev/null 2>&1
        echo "[INFO] Completed ${node}-${loop}."
    done
done
