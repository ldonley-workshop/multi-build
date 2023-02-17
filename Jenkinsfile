pipeline {
  agent none
  stages {
    stage('Build and Test') {
      parallel {
        stage('Build and Test Maven Project') {
          agent {
            kubernetes {
              label 'maven'
              yaml """
apiVersion: v1
kind: Pod
metadata:
  labels:
    app: maven
spec:
  containers:
  - name: maven
    image: maven:3.9.0-eclipse-temurin-17
    command:
    - cat
    tty: true
"""
            }
          }
          stages {
            stage('Build Maven Project') {
              steps {
                checkout scm
                container('maven') {
                  dir('maven-demo') {
                    sh 'mvn package'
                  }
                }
                archiveArtifacts artifacts: 'maven-demo/target/**/*.jar', fingerprint: true
              }
            }
            stage('Test Maven Project') {
              steps {
                container('maven') {
                  dir('maven-demo') {
                    sh 'mvn test'
                  }
                }
              }
            }
          }
        }
        stage('Build and Test Gradle Project') {
          agent {
            kubernetes {
              label 'gradle'
              yaml """
apiVersion: v1
kind: Pod
metadata:
  labels:
    app: gradle
spec:
  containers:
  - name: gradle
    image: gradle:jdk17
    command:
    - cat
    tty: true
"""
            }
          }
          stages {
            stage('Build Gradle Project') {
              steps {
                checkout scm
                container('gradle') {
                  dir('gradle-demo') {
                    sh './gradlew build'
                  }
                }
                archiveArtifacts artifacts: 'gradle-demo/build/**/*.jar', fingerprint: true
              }
            }
            stage('Test Gradle Project') {
              steps {
                container('gradle') {
                  dir('gradle-demo') {
                    sh './gradlew test'
                  }
                }
              }
            }
          }
        }
        stage('Build and Test Cargo Project') {
          agent {
            kubernetes {
              label 'cargo'
              yaml """
apiVersion: v1
kind: Pod
metadata:
  labels:
    app: cargo
spec:
  containers:
  - name: cargo
    image: rustlang/rust:nightly
    command:
    - cat
    tty: true
"""
            }
          }
          stages {
            stage('Build Cargo Project') {
              steps {
                checkout scm
                container('cargo') {
                  dir('cargo-demo') {
                    sh 'cargo build'
                  }
                }
                archiveArtifacts artifacts: 'cargo-demo/target/**/*.so', fingerprint: true
              }
            }
            stage('Test Cargo Project') {
              steps {
                container('cargo') {
                  dir('cargo-demo') {
                    sh 'cargo test'
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
