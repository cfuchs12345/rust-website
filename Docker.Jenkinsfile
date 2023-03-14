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
   
        booleanParam(
           name: "PushImage", 
           defaultValue: false)
    }

    stages {
        stage('Clean') {
            steps {
                 sh "docker image rm ${params.IMAGE_NAME}:latest"
                 sh "docker image prune -a -f"
            }
        }
        stage('pull artifact') {
            steps {
                copyArtifacts projectName: "${params.source_project}"

                unzip zipFile: "${params.artifact_file}"
            }
        }

        stage('Build image') {
            steps {
                 script {
               echo "Bulding docker images"
               def buildArgs = """\
                --build-arg UBUNTU_VERSION=${params.UBUNTU_VERSION} \
                --build-arg HTTP_PORT=8080 \
                -f Dockerfile \
                ."""
                docker.build(
                   "${params.IMAGE_NAME}:latest",
                   buildArgs)
            }
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