pipeline {
  agent none
  stages {
    stage('Build') {
      parallel {
        stage('Build Maven Project') {
          agent {
            kubernetes {
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
          steps {
            checkout scm
            container('maven') {
              sh 'cd maven-demo && mvn package'
            }
          }
        }
        stage('Build Gradle Project') {
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
          steps {
            checkout scm
            container('gradle') {
              sh 'cd gradle-demo && ./gradlew build'
            }
          }
        }
        stage('Build Cargo Project') {
          agent {
            kubernetes {
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
          steps {
            checkout scm
            container('cargo') {
              sh 'cd rust-demo && cargo build'
            }
          }
        }
      }
    }
  }
}
