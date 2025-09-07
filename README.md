# 🏥 Server API Monitor - Your Kubernetes Health Dashboard

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/apptolast/ServerApiMonitor)
[![Docker](https://img.shields.io/badge/docker-supported-blue.svg)](https://docker.com)
[![Kubernetes](https://img.shields.io/badge/kubernetes-compatible-orange.svg)](https://kubernetes.io)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-purple.svg)](https://github.com/apptolast/ServerApiMonitor)

## 🤔 What is this project?

Think of this project as a **digital health monitor for your applications** - just like how a hospital monitor shows a patient's vital signs, this tool shows the "vital signs" of all your software applications running in the cloud.

**In simple terms:** This is a web service that automatically checks if your applications are running properly, having problems, or completely stopped. It gives you an easy way to see the health status of everything at once, instead of checking each application one by one.

**Real-world analogy:** Imagine you're a building manager responsible for 100 offices. Instead of walking to each office to see if people are working, you have a central dashboard that shows which offices are:
- ✅ **Running normally** (people working)
- ⚠️ **Having issues** (some problems but still working)  
- ❌ **Completely down** (empty office)
- ⏳ **Starting up** (people arriving)

## 🎯 What is it for?

### Primary Use Cases

**For System Administrators:**
- Monitor all applications in your company's cloud infrastructure
- Quickly identify which services are having problems
- Get detailed information about application restarts and failures
- Have a central place to check everything instead of logging into multiple systems

**For DevOps Teams:**
- Automate health monitoring of containerized applications
- Get JSON data that can be integrated with other monitoring tools
- Monitor Kubernetes clusters without complex setup
- Troubleshoot deployment issues faster

**For IT Managers:**
- Get a high-level view of system health
- Understand how many applications are running vs. having problems
- Make informed decisions about infrastructure stability
- Monitor uptime and reliability metrics

## ✨ Key Features

### 🔍 **Automatic Discovery**
- Automatically finds and monitors ALL applications in your system
- No need to manually configure each application
- Works with any type of application (web services, databases, background jobs, etc.)

### 🌐 **Simple Web Interface**
- Easy-to-use web address you can visit in any browser
- Returns information in a format that both humans and other programs can read
- Three main endpoints:
  - Check if the monitor itself is working
  - Get basic information about the service
  - Get detailed health status of all applications

### 📊 **Detailed Health Information**
For each application, you get:
- **Name and location** (which namespace/team owns it)
- **Current status** (Running, Failed, Pending, etc.)
- **Health status** (Ready or Not Ready)
- **Restart count** (how many times it had to be restarted)
- **Container details** (individual components within each application)

### 🔄 **Real-time Monitoring**
- Information is always up-to-date when you request it
- No delays or outdated information
- Connects directly to your Kubernetes cluster for live data

### 🚀 **Easy Deployment**
- Can run inside your Kubernetes cluster (recommended)
- Can run on your local computer for development/testing
- Pre-built Docker container - no compilation needed
- Automatic configuration - detects where it's running

## 📥 Installation Guide

### Option 1: Running with Docker (Easiest)

**What you need:**
- A computer with Docker installed
- Access to a Kubernetes cluster
- Basic command line knowledge

**Steps:**

1. **Install Docker** (if not already installed)
   - Visit [docker.com](https://docker.com) and download Docker Desktop
   - Follow the installation instructions for your operating system

2. **Get the application**
   ```bash
   # Download the pre-built application
   docker pull apptolast/server-api-monitor:latest
   ```

3. **Run the application**
   ```bash
   # Start the health monitor
   docker run -p 3000:3000 apptolast/server-api-monitor:latest
   ```

4. **Test if it's working**
   - Open your web browser
   - Go to: `http://localhost:3000/health`
   - You should see a response showing the service is healthy

### Option 2: Running in Kubernetes (Recommended for Production)

**What you need:**
- A Kubernetes cluster where you can deploy applications
- kubectl command-line tool configured
- Basic understanding of Kubernetes deployments

**Steps:**

1. **Create a deployment file**
   Save this as `health-monitor-deployment.yaml`:
   ```yaml
   apiVersion: apps/v1
   kind: Deployment
   metadata:
     name: health-monitor
   spec:
     replicas: 1
     selector:
       matchLabels:
         app: health-monitor
     template:
       metadata:
         labels:
           app: health-monitor
       spec:
         containers:
         - name: health-monitor
           image: apptolast/server-api-monitor:latest
           ports:
           - containerPort: 3000
   ---
   apiVersion: v1
   kind: Service
   metadata:
     name: health-monitor-service
   spec:
     selector:
       app: health-monitor
     ports:
     - port: 80
       targetPort: 3000
     type: LoadBalancer
   ```

2. **Deploy to Kubernetes**
   ```bash
   kubectl apply -f health-monitor-deployment.yaml
   ```

3. **Find the service URL**
   ```bash
   kubectl get services health-monitor-service
   ```

### Option 3: Building from Source (For Developers)

**What you need:**
- Rust programming language installed
- Git for downloading the source code
- A Kubernetes cluster for testing

**Steps:**

1. **Install Rust**
   - Visit [rustup.rs](https://rustup.rs) and follow the installation instructions

2. **Download the source code**
   ```bash
   git clone https://github.com/apptolast/ServerApiMonitor.git
   cd ServerApiMonitor
   ```

3. **Build the application**
   ```bash
   cargo build --release
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

## 🚀 How to Use

### Basic Usage

Once the service is running, you have three main ways to get information:

#### 1. **Check Service Status**
**URL:** `http://your-server:3000/health`

**What it does:** Tells you if the monitoring service itself is working properly.

**Example response:**
```json
{
  "name": "Health Dashboard API",
  "version": "0.1.0", 
  "status": "healthy"
}
```

#### 2. **Get Service Information**
**URL:** `http://your-server:3000/`

**What it does:** Shows basic information about the monitoring service.

**Example response:**
```json
{
  "name": "Health Dashboard API",
  "version": "0.1.0",
  "status": "running"
}
```

#### 3. **Get All Application Health Status**
**URL:** `http://your-server:3000/pods`

**What it does:** Shows detailed health information for all applications in your cluster.

**Example response:**
```json
{
  "total_pods": 25,
  "running_pods": 20,
  "failed_pods": 2,
  "pending_pods": 3,
  "pods": [
    {
      "name": "web-application-1",
      "namespace": "production",
      "status": "Running",
      "ready": true,
      "restart_count": 0,
      "containers": [
        {
          "name": "web-server",
          "ready": true,
          "restart_count": 0
        }
      ]
    },
    {
      "name": "database-backup-job",
      "namespace": "production", 
      "status": "Failed",
      "ready": false,
      "restart_count": 5,
      "containers": [
        {
          "name": "backup-script",
          "ready": false,
          "restart_count": 5
        }
      ]
    }
  ]
}
```

### Understanding the Results

**Summary Information:**
- `total_pods`: Total number of applications found
- `running_pods`: Applications that are working normally
- `failed_pods`: Applications that have crashed or failed
- `pending_pods`: Applications that are starting up

**Individual Application Information:**
- `name`: The name of the application
- `namespace`: Which team/project owns this application
- `status`: Current state (Running, Failed, Pending, etc.)
- `ready`: Whether the application is ready to handle requests
- `restart_count`: How many times this application had to be restarted
- `containers`: Individual components within the application

### Practical Examples

#### Example 1: Quick Health Check
**Scenario:** You want to quickly see if everything is running fine.

**Action:** Visit `http://your-server:3000/pods` and look at the summary:
- If `failed_pods` is 0, everything is healthy
- If `failed_pods` is greater than 0, some applications need attention

#### Example 2: Finding Problem Applications
**Scenario:** You know something is wrong and want to find which application is causing problems.

**Action:** Visit `http://your-server:3000/pods` and look for applications where:
- `status` is "Failed" 
- `ready` is false
- `restart_count` is high (more than 5-10 restarts might indicate a persistent problem)

#### Example 3: Monitoring During Deployment
**Scenario:** You're deploying new applications and want to see them start up.

**Action:** Refresh `http://your-server:3000/pods` periodically and watch for:
- New applications appearing in the list
- Applications changing from "Pending" to "Running"
- `ready` status changing from false to true

## 📸 Screenshots/Demo

### API Response Examples

**Healthy System Overview:**
```
✅ Total Applications: 15
✅ Running: 15  
❌ Failed: 0
⏳ Pending: 0
```

**System with Issues:**
```
⚠️ Total Applications: 15
✅ Running: 12
❌ Failed: 2  
⏳ Pending: 1
```

### Sample Dashboard Integration

*Note: This service provides the data through a web API. You can integrate it with monitoring dashboards like Grafana, or build your own web interface using the JSON data provided.*

**Example integration possibilities:**
- **Grafana Dashboard:** Create charts showing application health over time
- **Slack Alerts:** Send notifications when applications fail
- **Custom Web Interface:** Build a visual dashboard for your team
- **Automated Reports:** Generate daily/weekly health reports

## ❓ Frequently Asked Questions (FAQ)

### General Questions

**Q: Do I need to know programming to use this?**
A: No! You can use this tool by simply visiting web addresses in your browser. The installation requires some basic command-line knowledge, but day-to-day usage is just clicking links.

**Q: Will this slow down my applications?**
A: No. This tool only reads information from Kubernetes - it doesn't change or interfere with your applications in any way.

**Q: How often does the information update?**
A: The information is real-time. Every time you visit the web address, it gets the current status directly from your Kubernetes cluster.

### Installation Questions

**Q: What is Kubernetes and do I need it?**
A: Kubernetes is a system for running applications in containers (think of them as lightweight virtual machines). Yes, you need a Kubernetes cluster for this tool to monitor. If you don't have one, this tool won't be useful for you.

**Q: Can I run this on Windows/Mac/Linux?**
A: Yes! The Docker version works on all operating systems. The Kubernetes version works regardless of the operating system since it runs inside the cluster.

**Q: Do I need special permissions?**
A: Yes, the application needs permission to read information from your Kubernetes cluster. If you're running it inside Kubernetes, it can often get these permissions automatically. If you're running it locally, you'll need a kubeconfig file with read access.

### Usage Questions

**Q: What does "namespace" mean?**
A: A namespace is like a folder that groups related applications together. For example, you might have a "production" namespace for live applications and a "testing" namespace for experimental applications.

**Q: Why would an application need to restart?**
A: Applications restart for many reasons: software bugs, running out of memory, configuration changes, or updates. Occasional restarts (1-2) are normal, but many restarts might indicate a problem.

**Q: What's the difference between "status" and "ready"?**
A: "Status" tells you the overall state (Running, Failed, etc.). "Ready" tells you if the application can actually handle work. An application might be "Running" but not "Ready" if it's still starting up.

### Troubleshooting

**Q: I get "connection refused" errors**
A: This usually means:
- The service isn't running (check if the container started successfully)
- You're using the wrong port (make sure you're accessing port 3000)
- There's a firewall blocking the connection

**Q: I see "Error getting pods" in the response**
A: This means the service can't connect to your Kubernetes cluster. Check:
- Are you running in a Kubernetes cluster with proper permissions?
- Is your kubeconfig file correct (if running locally)?
- Does the service account have read permissions?

**Q: The service starts but shows no applications**
A: This could mean:
- You're looking at an empty Kubernetes cluster
- The service doesn't have permission to read from all namespaces
- You're connected to the wrong Kubernetes cluster

## 🤝 Contributing

We welcome contributions from everyone, regardless of your technical background! Here are ways you can help:

### For Non-Technical Contributors

**📝 Documentation**
- Help improve this README by suggesting clearer explanations
- Add more real-world examples and use cases
- Translate documentation to other languages
- Create video tutorials or guides

**🐛 Testing and Feedback**
- Try installing and using the application
- Report any confusing instructions or error messages
- Suggest new features that would be helpful
- Share your use cases and how you're using the tool

**🎨 Design and User Experience**
- Suggest improvements to the API response format
- Propose ideas for a web-based user interface
- Create mockups for dashboards or visualizations
- Help make error messages more user-friendly

### For Technical Contributors

**💻 Code Contributions**
- Add new features or endpoints
- Improve error handling and logging
- Optimize performance
- Add automated tests

**🚀 Infrastructure**
- Improve Docker container optimization
- Add Kubernetes deployment examples
- Create Helm charts for easier deployment
- Set up CI/CD pipelines

**📊 Monitoring and Observability**
- Add metrics and monitoring capabilities
- Implement health check endpoints
- Add support for other monitoring systems
- Create alerting capabilities

### How to Get Started

1. **Fork the repository** on GitHub
2. **Create an issue** describing what you'd like to work on
3. **Ask questions** - we're happy to help you get started
4. **Make your changes** in small, focused commits
5. **Submit a pull request** with a clear description of your changes

### Code of Conduct

We're committed to providing a welcoming and inclusive environment for all contributors. Please:
- Be respectful and constructive in all interactions
- Help others learn and grow
- Focus on what's best for the community
- Ask for help when you need it

### Getting Help

- **GitHub Issues:** Ask questions or report problems
- **Discussions:** Share ideas and get community feedback
- **Email:** Contact the maintainers directly for sensitive issues

## 📋 Technical Details

*For users who want to understand the technical implementation:*

**Built with:**
- **Rust** - Systems programming language for performance and safety
- **Axum** - Modern web framework for the HTTP API
- **Tokio** - Async runtime for handling concurrent requests
- **Kube-rs** - Kubernetes client library for cluster communication

**Architecture:**
- RESTful HTTP API design
- Async/await for non-blocking operations
- Automatic service discovery in Kubernetes
- JSON response format for easy integration

**Security:**
- Runs with minimal privileges
- Read-only access to Kubernetes API
- No data persistence (stateless design)
- Container security best practices

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🏷️ Version History

- **v0.1.0** - Initial release with basic pod monitoring functionality

---

**Made with ❤️ for the Kubernetes community**

*If you find this project helpful, please consider giving it a ⭐ on GitHub!*
