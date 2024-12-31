#include "fheco/fheco.hpp"
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <random>
#include <string>
#include <vector>
#include <unordered_map>
using namespace std;
using namespace fheco;

int getRandomNumber(int range) {
    static std::random_device rd;
    static std::mt19937 rng(rd());
    std::uniform_int_distribution<int> dist(0, range - 1);
    return dist(rng);
}

int64_t getRandomInt64(int64_t range) {
    static std::random_device rd;
    static std::mt19937_64 rng(rd());
    std::uniform_int_distribution<int64_t> dist(0, range - 1);
    return dist(rng);
}

ofstream outputFile;
int idCounter = 0;

string generateCiphertextID() {
    idCounter = idCounter + 10;
    int id = getRandomInt64(idCounter);
    return "c_" + to_string(id);
}

Ciphertext createCiphertext() {
    string id = generateCiphertextID();
    string is_signed = " 0 ";
    int randomValue = getRandomNumber(10);
    outputFile << id << " 1" << is_signed << randomValue << endl;
    return Ciphertext(id);
}

Ciphertext treeGenerator(int originalDepth, int maxDepth, int& seed, const string& regime) {
    if (maxDepth == 0) {
        return createCiphertext();
    }

    Ciphertext lhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);
    Ciphertext rhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);

    int randomOp = getRandomNumber(2);
    if (randomOp == 0) {
        return lhs + rhs;
    } else {
        return lhs * rhs;
    }
}

void fhe(int depth, int iteration, string regime) {
    int seed = 9100 + (iteration - 1) * 100 + (depth * 100) + iteration;
    Ciphertext result = treeGenerator(depth, depth, seed, regime);
    result.set_output("result");
}

void print_bool_arg(bool arg, const string &name, ostream &os)
{
  os << (arg ? name : "no_" + name);
}

int main(int argc, char **argv) {
    auto axiomatic = false;
    auto window = 0;
    bool cse = true;
    bool const_folding = true;
    /***************************/
    outputFile.open("fhe_io_example.txt");
    if (!outputFile.is_open()) {
        throw runtime_error("Failed to open fhe_io_example.txt for writing");
    }
    /***************************/
    /**************************/
    bool call_quantifier = true;
    bool vectorized = true;
    /**************************/
    // Argument validation
    if (argc < 4) {
        throw invalid_argument("Some arguments are lacking, needed arguments are: depth, iteration, regime");
    }
    int depth = stoi(argv[1]);
    int iteration = stoi(argv[2]);
    string regime = argv[3];
    if (cse) {
        Compiler::enable_cse();
        Compiler::enable_order_operands();
    } else {
        Compiler::disable_cse();
        Compiler::disable_order_operands();
    }

    if (const_folding) {
        Compiler::enable_const_folding();
    } else {
        Compiler::disable_const_folding();
    }
    /********************************************/
    chrono::high_resolution_clock::time_point t;
    chrono::duration<double, milli> elapsed;
    t = chrono::high_resolution_clock::now();
    string func_name = "fhe";
    size_t slot_count = 1;
    if(vectorized){
        const auto &func = Compiler::create_func(func_name, slot_count, 20, false, true);
        fhe(depth, iteration, regime);
        string gen_name = "_gen_he_" + func_name;
        string gen_path = "he/" + gen_name;
        // Creating header and source files
        ofstream header_os(gen_path + ".hpp");
        if (!header_os) {
            throw logic_error("Failed to create header file");
        }
        ofstream source_os(gen_path + ".cpp");
        if (!source_os) {
            throw logic_error("Failed to create source file");
        }
        cout << " window is " << window << endl;
        Compiler::gen_vectorized_code(func, window /*is_structured*/);  // add a flag to specify if the benchmark is structured or no
        auto ruleset = Compiler::Ruleset::ops_cost;
        auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
        Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);
        Compiler::gen_he_code(func, header_os, gen_name + ".hpp", source_os);
        /************/elapsed = chrono::high_resolution_clock::now() - t;
        cout << elapsed.count() << " ms\n";
        if (call_quantifier)
        {
            util::Quantifier quantifier{func};
            quantifier.run_all_analysis();
            quantifier.print_info(cout);
        }
    }else{
        const auto &func = Compiler::create_func(func_name, slot_count, 20, false, true);
        fhe(depth, iteration, regime);
        string gen_name = "_gen_he_" + func_name;
        string gen_path = "he/" + gen_name;
        // Creating header and source files
        ofstream header_os(gen_path + ".hpp");
        if (!header_os) {
            throw logic_error("Failed to create header file");
        }
        ofstream source_os(gen_path + ".cpp");
        if (!source_os) {
            throw logic_error("Failed to create source file");
        }
        cout << " window is " << window << endl;
        auto ruleset = Compiler::Ruleset::ops_cost;
        auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
        Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);
        /************/elapsed = chrono::high_resolution_clock::now() - t;
        cout << elapsed.count() << " ms\n";
        if (call_quantifier)
        {
            util::Quantifier quantifier{func};
            quantifier.run_all_analysis();
            quantifier.print_info(cout);
        }
    }
    return 0;
}