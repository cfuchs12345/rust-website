pipeline {
    agent {label 'rust-slave'}

    parameters {
        string(
        name: "CLEAN_UP", 
        defaultValue: 'false', 
        description: 'Do cleanup before build')
    }
    options {
        copyArtifactPermission('docker/Docker-Webseite');
    }
    stages {
        stage('checkout') {
            steps {
                checkout poll: false, scm: scmGit(
                    branches: [[name: '*/master']], 
                    extensions: [],
                    userRemoteConfigs: 
                    [[credentialsId: 'git-jenkins', 
                    url: 'https://gitea.home-of-the-fox.duckdns.org/cfuchs113/rust-website.git'
                    ]])
            }
        }
        stage('Init') {
            steps {
                sh "rustup default stable"
                sh "apt install npm"
                sh "npm i -g javascript-obfuscator"
            }
        }
        stage('Clean') {
             when {
                expression { params.CLEAN_UP != 'false' }
            }
            steps {
                sh "cargo clean"
                sh "cargo clean --release"
            }
        }
        stage('Build') {
            steps {
                sh "cargo build --release"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "cargo clippy --all"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "cargo fmt --all"
            }
        }
        stage('Doc') {
            steps {
                sh "cargo doc"
                // We run a python `SimpleHTTPServer` against
                // /var/lib/jenkins/jobs/<repo>/branches/master/javadoc to
                // display our docs
                step([$class: 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll: false])
            }
        }
        stage("Obfuscate Code") {
            steps {
                sh "javascript-obfuscator .\static\scripts\page_pretty.js --output .\static\scripts\page.js"
                sh "javascript-obfuscator .\static\scripts\bootstrap.bundle_pretty.js --output .\static\scripts\bootstrap.bundle.js"
            }
        }
        stage("Create Artifact") {
            steps {
                zip zipFile: "target/rust_website_webserver.zip", archive: true, dir: "target/release", overwrite: true, glob: "rustwebserver"
            }
        }
    }
    post{
        always{
            archiveArtifacts artifacts: 'target/rust_website_webserver.zip', fingerprint: true

            emailext body: 'Build executed',
            recipientProviders: [developers(), requestor()],
            subject: 'jenkins build ${JOB_DESCRIPTION}: ${BUILD_STATUS}',
            to: 'christopher@christopherfuchs.de'
        }
    }
}