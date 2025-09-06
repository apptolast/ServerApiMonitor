use serde::Serialize;

// API info
#[derive(Serialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}

// Pod info
#[derive(Serialize, Debug)]
pub struct PodInfo {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub ready: bool,
    pub restart_count: i32,
    pub containers: Vec<ContainerInfo>,
}

// Container info
#[derive(Serialize, Debug)]
pub struct ContainerInfo {
    pub name: String,
    pub ready: bool,
    pub restart_count: i32,
}

// Resume about cluster
#[derive(Serialize)]
pub struct ClusterSummary {
    pub total_pods: usize,
    pub running_pods: usize,
    pub failed_pods: usize,
    pub pending_pods: usize,
    pub pods: Vec<PodInfo>,
}