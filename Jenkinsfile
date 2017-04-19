def branches = [:]
def names = nodeNames();

for (int i = 0; i < names.size(); i++) {
	def nodeName = names[i];
	branches["node(" + nodeName + ")"] = {
		node(nodeName) {
			stage('Checkout') {
				checkout scm;
			}
			stage('Version') {
				sh 'rustc --version';
				sh 'cargo --version';
			}
			stage('Build') {
				sh 'cargo build --verbose';
			}
			stage('Test') {
				sh 'cargo test --verbose';
			}
			stage('Bench') {
				sh 'cargo bench --verbose';
			}
			stage('Doc') {
				sh 'cargo doc --verbose';
			}
		}
	}
}

// Now we trigger all branches
parallel branches

// This method collects a list of Node names from the current Jenkins instance
@NonCPS
def nodeNames() {
	return jenkins.model.Jenkins.instance.nodes.collect { node -> node.name }
}

