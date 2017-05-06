def branches = [:]
def names = nodeNames();

for (int i = 0; i < names.size(); i++) {
	def nodeName = names[i];
	branches["node(" + nodeName + ")"] = {
		node(nodeName) {
			withEnv(["CARGO_FLAGS=--features graphviz"]) {
				stage('Checkout') {
					checkout scm;
				}
				stage('Version') {
					sh 'rustc --version';
					sh 'cargo --version';
				}
				stage('Build') {
					sh 'cargo build --verbose $CARGO_FLAGS';
				}
				stage('Test') {
					sh 'cargo test --verbose $CARGO_FLAGS';
				}
				stage('Bench') {
					sh 'cargo bench -- verbose $CARGO_FLAGS';
				}
				stage('Doc') {
					sh 'cargo doc --verbose $CARGO_FLAGS';
				}
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

