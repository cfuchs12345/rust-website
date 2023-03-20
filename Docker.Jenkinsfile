pipeline {
    agent {label 'docker-slave'}

     parameters {
            string(
            name: "Branch_Name", 
            defaultValue: 'master', 
            description: '')
            string(
            name: "source_project",
            defaultValue: 'rust/(Rust) Webseite',
            description: 'source project that generates the rust executable',
            )
             string(
            name: "artifact_file",
            defaultValue: 'target/rust_website_webserver.zip',
            description: 'artifact name of the source project',
            )        
            string(
            name: "UBUNTU_VERSION", 
            defaultValue: 'focal', 
            description: 'version of ubuntu base image')
            string(
            name: "IMAGE_NAME", 
            defaultValue: 'docker-rust-website', 
            description: 'name of the image')
            string(
            name: "DOCKER_REGISTRY",
            defaultValue: 'docker.registry.lan:5000',
            description: 'registry location'
            )
    }

    environment {
        registry = ""
        registryCredential = ''
        dockerImage = ''
    }


    stages {
        stage('Clean') {
            steps {
                 sh "docker image rm ${params.IMAGE_NAME}:latest || true"
                 sh "docker image prune -a -f || true"
                 sh "rm rustwebserver || true"
                 sh "rm ${params.artifact_file} || true"
            }
        }
        stage('pull artifact') {
            steps {
                copyArtifacts projectName: "${params.source_project}", selector: lastSuccessful()

                unzip zipFile: "${params.artifact_file}"
            }
        }
         stage('checkout') {
            steps {
                dir ('private') {
                    checkout poll: false, scm: scmGit(
                        branches: [[name: '*/master']], 
                        extensions: [],
                        userRemoteConfigs: 
                        [[credentialsId: 'git-jenkins', 
                        url: 'https://gitea.home-of-the-fox.duckdns.org/cfuchs113/rust-website-private-config.git'
                        ]])
                }
            }
        }

        stage('Build image') {
            steps {
                sh "cp private/translations.json ."
                sh "cp private/.env ."
                sh "cp private/labels ."

                script {
                echo "Bulding docker images"
                def buildArgs = """\
                --build-arg UBUNTU_VERSION=${params.UBUNTU_VERSION} \
                --build-arg HTTP_PORT=8080 \
                -f Dockerfile \
                --no-cache \
                ."""
                docker.build(
                   "${params.IMAGE_NAME}:$BUILD_NUMBER",
                   buildArgs)
                }
            }
        }
        stage('Tag image') {
            steps {
                script {
                    echo "Tagging docker image"
                    sh "docker tag ${params.IMAGE_NAME}:$BUILD_NUMBER ${params.DOCKER_REGISTRY}/${params.IMAGE_NAME}:latest";
                }
            }
        }
        stage('Deploy Image') {
            steps{
                script {
                    echo "Push docker image to local registry"
                    sh "docker push ${params.DOCKER_REGISTRY}/${params.IMAGE_NAME}:latest"
                    echo "Deleting local image"
                    sh "docker rmi ${params.IMAGE_NAME}:$BUILD_NUMBER || true"
                    sh "docker rmi ${params.DOCKER_REGISTRY}/${params.IMAGE_NAME}:$BUILD_NUMBER || true"
                }
            }
        }
        stage('Create Artifact') {
            steps {
                echo "creating artifact with docker scripts and config"
            }
        }
    }
    post{
        always{
            emailext body: 'Build executed',
            recipientProviders: [developers(), requestor()],
            subject: 'jenkins build ${JOB_DESCRIPTION}: ${BUILD_STATUS}',
            to: 'christopher@christopherfuchs.de'
        }
    }
}