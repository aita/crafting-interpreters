cmake_minimum_required(VERSION 3.0.0)
project(lox VERSION 0.1.0)

file(GLOB SOURCES *.c *.h)

add_executable(lox ${SOURCES})
