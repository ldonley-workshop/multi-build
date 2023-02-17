pipeline {
  agent none
  stages {
    stage('Build and Test') {
      parallel {
        stage('Maven') {
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
            stage('Maven build') {
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
            stage('Maven test') {
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
        stage('Gradle') {
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
            stage('Gradle build') {
              steps {
                checkout scm
                container('gradle') {
                  dir('gradle-demo') {
                    sh './gradlew build'
                  }
                }
                archiveArtifacts artifacts: 'gradle-demo/lib/build/**/*.jar', fingerprint: true
              }
            }
            stage('Gradle test') {
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
        stage('Cargo') {
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
            stage('Cargo build') {
              steps {
                checkout scm
                container('cargo') {
                  dir('cargo-demo') {
                    sh 'cargo build'
                  }
                }
              }
            }
            stage('Cargo test') {
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
