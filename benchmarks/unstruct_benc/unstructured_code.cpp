#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;


void generate_code_file() {
    const char *command = "python3 ./script.py";
    int result = system(command);
    if (result != 0) {
        cerr << "Error: Failed to execute the Python script" << endl;
    }
}

int main(int argc, char **argv) {
    // Compile number_tree_generator.cpp
    int compile_result = system("g++ number_tree_generator.cpp -o number_tree_generator");
    if (compile_result != 0) {
        cerr << "Error: Failed to compile number_tree_generator.cpp" << endl;
        return 1;
    }

    // Execute number_tree_generator to create the expression file
    int exec_result = system("./number_tree_generator");
    if (exec_result != 0) {
        cerr << "Error: Failed to execute number_tree_generator" << endl;
        return 1;
    }

    // Open the generated file
    ifstream expressionFile("generated_expression.txt");
    if (!expressionFile) {
        cerr << "Error: Could not open generated_expression.txt" << endl;
        return 1;
    }

    // Read the expression
    string expression;
    getline(expressionFile, expression);
    expressionFile.close();

    // Output the read expression (for debugging or information purposes)
    cout << "Generated expression: " << expression << endl;

    // Call the Python script to generate the final C++ file
    generate_code_file();

    return 0;
}
