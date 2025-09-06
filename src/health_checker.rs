use kube::{Api, Client};
use k8s_openapi::api::core::v1::Pod;
use crate::models::{PodInfo, ContainerInfo, ClusterSummary};
use tracing::{info, warn};


// Obtain info of all pods of the cluster
pub async fn get_cluster_pods(client: Client) -> anyhow::Result<ClusterSummary> {

    // Get Pod type for the cluster of Kubernetes
    let pods: Api<Pod> = Api::all(client);

    info!(" Consulting pods in the cluster");

    // List all the pods
    let pod_list = pods.list(&Default::default()).await?;

    // Variables for interesting data
    let mut podInfos = Vec::new();
    let mut runningCount = 0;
    let mut failedCount = 0;
    let mut pendingCount = 0;


    // Running each pod in detail
    for pod in pod_list {
        // Get metadata of the single pod
        // Closure function |params| body -> return result
        let name = pod.metadata.name.unwrap_or_else(|| "uknown".to_string());
        let namespace = pod.metadata.namespace.unwrap_or_else(|| "uknown".to_string());

        //Get status of the pod
        let pod_status = pod.status.as_ref();
        let phase = pod_status
            .and_then(|s| s.phase.clone())
            .unwrap_or_else(|| "uknown".to_string());

        // Count for state
        match phase.as_str() {
            "Running" => runningCount += 1,
            "Failed" => failedCount += 1,
            "Pending" => pendingCount += 1,
            _ => {}
        }

        // Extract info containers
        let container_statuses = pod_status
            .and_then(|s| s.container_statuses.clone())
            .unwrap_or_default();

        // Containers
        let containers: Vec<ContainerInfo> = container_statuses
            .iter()
            .map(|cs| ContainerInfo {
                name: cs.name.clone(),
                ready: cs.ready,
                restart_count: cs.restart_count,
            })
            .collect();

        // Check if pod is ready
        let ready = containers.iter().all(|c| c.ready);
        let total_restarts: i32 = containers.iter().map(|c| c.restart_count).sum();

        // Info pods

        podInfos.push(PodInfo {
            name,
            namespace,
            status: phase,
            ready,
            restart_count: total_restarts,
            containers,
        });
    }

      info!("Find {} pods", podInfos.len());


    //Get the sumarry
    Ok(ClusterSummary {
        total_pods: podInfos.len(),
        running_pods: runningCount,
        failed_pods: failedCount,
        pending_pods: pendingCount,
        pods: podInfos,
    })
}