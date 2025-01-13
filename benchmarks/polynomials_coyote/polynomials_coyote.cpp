#include "fheco/fheco.hpp"
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <ostream>
#include <stdexcept>
#include <string>
#include <vector>
#include <sstream>
#include <random>
using namespace std;
using namespace fheco;

std::ofstream outFile("fhe_io_example.txt", std::ios::trunc);

/*************************************************************** */
// Function for generating random numbers
int getRandomNumber(int range) {
    static std::random_device rd;   // Seed generator
    static std::mt19937 rng(rd());  // Random number generator
    std::uniform_int_distribution<int> dist(0, range);
    return dist(rng);
}

int64_t getRandomInt64(int64_t range) {
    static std::random_device rd;         // Seed generator
    static std::mt19937_64 rng(rd());     // 64-bit Mersenne Twister generator
    std::uniform_int_distribution<int64_t> dist(0, range - 1);  // Exclusive upper bound
    return dist(rng);
}


/**************************************************************** */
// C++ equivalent of the treeGenerator function
Ciphertext treeGenerator(int originalDepth, int maxDepth, int& seed, const std::string& regime) {
    std::string localString = "";
    
    if (originalDepth == maxDepth || maxDepth == originalDepth - 1) {
        int randomNum = (regime == "100-100") ? getRandomNumber(1) : getRandomNumber(1);
        Ciphertext lhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);
        Ciphertext rhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);

        if (randomNum == 1) {
            return lhs + rhs;
        } else {
            return lhs * rhs;
        }
    }

    if (maxDepth > 0) {
        int randomNum = (regime == "50-50") ? getRandomNumber(3) : (regime == "100-50") ? getRandomNumber(1) : getRandomNumber(0);
        seed += 1;

        if (randomNum > 1) {
            localString = std::to_string(getRandomNumber(1024));
            seed += 1;
            std::cout << originalDepth + 1 - maxDepth << std::endl;
            int random = getRandomNumber(100);
            string randomString = "c_" + std::to_string(std::rand() % 1000);
            outFile << randomString << " 1 0 " << std::to_string(random) << std::endl; // in "1 0" : 1 for ciphertext not plaintext , and 0 for signed or not signed
            return Ciphertext(randomString);
        } else {
            Ciphertext lhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);
            Ciphertext rhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);

            if (randomNum == 1) {
                return lhs + rhs;
            } else {
                return lhs * rhs;
            }
        }
    } else {
        //integer endNode = (1024);
        seed += 1;
        int random = getRandomNumber(100);
        string randomString = "c_" + std::to_string(std::rand() % 1000);
        outFile << randomString << " 1 0 " << std::to_string(random) << std::endl; // in "1 0" : 1 for ciphertext not plaintext , and 0 for signed or not signed
        return Ciphertext(randomString);
    }
}
/*****************************************************/
void fhe(int depth, int iteration, std::string regime) {
    int seed = 9100 + (iteration - 1) * 100 + (depth * 100) + iteration;
    Ciphertext result = treeGenerator(depth, depth, seed, regime);
    result.set_output("result");

    const std::string filePath = "fhe_io_example.txt";

    // Open the input file for reading
    std::ifstream inputFile(filePath);
    if (!inputFile.is_open()) {
        std::cerr << "Error: Unable to open file for reading.\n";
        return;
    }

    int lineCount = 0;
    std::string line;
    std::stringstream fileContent;

    while (std::getline(inputFile, line)) {
        lineCount++;
        fileContent << line << '\n';
    }
    inputFile.close();

    // Convert file content to string and remove any leading newline
    std::string fileData = fileContent.str();
    if (!fileData.empty() && fileData[0] == '\n') {
        fileData.erase(0, 1);
    }

    // Open the file for writing and overwrite its content
    std::ofstream outputFile(filePath, std::ios::trunc);
    if (!outputFile.is_open()) {
        std::cerr << "Error: Unable to open file for writing.\n";
        return;
    }

    outputFile << "1 " << std::to_string(lineCount) << " 1\n";
    outputFile << fileData;

    outputFile.close();
}

/***************************************************/
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
        int benchmark_type = 10;  // output_number = 1  , structured = 0
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
        Compiler::gen_vectorized_code(func, window, benchmark_type);  // add a flag to specify if the benchmark is structured or no
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