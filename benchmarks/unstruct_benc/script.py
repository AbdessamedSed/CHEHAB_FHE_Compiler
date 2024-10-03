import os
import time

def generate_cpp_file(expression):
    cpp_content = f"""
#include "fheco/fheco.hpp"

using namespace std;
using namespace fheco;
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

void fhe()
{{
    Ciphertext output;

    output = ({expression});

    output.set_output("output");
}}


void print_bool_arg(bool arg, const string &name, ostream &os)
{{
    os << (arg ? name : "no_" + name);
}}

int main(int argc, char **argv)
{{
    bool vectorized = true;
    if (argc > 1)
        vectorized = stoi(argv[1]);

    int window = 1;
    if (argc > 2)
        window = stoi(argv[2]);

    bool call_quantifier = false;
    if (argc > 3)
        call_quantifier = stoi(argv[3]);

    bool cse = true;
    if (argc > 4)
        cse = stoi(argv[4]);

    bool const_folding = true;
    if (argc > 5)
        const_folding = stoi(argv[5]);

    if (cse)
    {{
        Compiler::enable_cse();
        Compiler::enable_order_operands();
    }}
    else
    {{
        Compiler::disable_cse();
        Compiler::disable_order_operands();
    }}

    if (const_folding)
        Compiler::enable_const_folding();
    else
        Compiler::disable_const_folding();

    chrono::high_resolution_clock::time_point t;
    chrono::duration<double, milli> elapsed;
    t = chrono::high_resolution_clock::now();
    string func_name = "fhe";
    const auto &func = Compiler::create_func(func_name, 1, 20, false, true);
    fhe();

    if (vectorized)
    {{
        cout << " window is " << window << endl;
        Compiler::gen_vectorized_code(func, window);
    }}
    else
    {{
        string gen_name = "_gen_he_" + func_name;
        string gen_path = "he/" + gen_name;
        ofstream header_os(gen_path + ".hpp");
        if (!header_os)
            throw logic_error("failed to create header file");

        ofstream source_os(gen_path + ".cpp");
        if (!source_os)
            throw logic_error("failed to create source file");

        if (call_quantifier)
        {{
            util::Quantifier quantifier{{func}};
            quantifier.run_all_analysis();
            quantifier.print_info(cout);
        }}
    }}

    elapsed = chrono::high_resolution_clock::now() - t;
    cout << elapsed.count() << " ms\\n";

    return 0;
}}
    """

    
    cpp_file_path = f"benchmark.cpp"

    with open(cpp_file_path, "w") as cpp_file:
        cpp_file.write(cpp_content)

    print(f"Generated C++ file: {cpp_file_path}")

def main():
    try:
        with open("generated_expression.txt", "r") as file:
            expression = file.readline().strip()

        generate_cpp_file(expression)
    except FileNotFoundError:
        print("Error: 'generated_expression.txt' not found.")
        return 1

    return 0

if __name__ == "__main__":
    exit(main())
