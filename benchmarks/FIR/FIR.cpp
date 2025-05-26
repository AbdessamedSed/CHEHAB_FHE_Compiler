#include "fheco/fheco.hpp"

using namespace std;
using namespace fheco;
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector> 
#include <cmath>
#include "../global_variables.hpp" 

void fhe(int signal_len) {

    std::vector<Ciphertext> signal(signal_len);
    std::vector<integer> filter = {1, 2, 3};
    std::vector<Ciphertext> output(signal_len);
    int filter_len = filter.size();

    for (int i = 0 ; i < signal_len ; i++) {
        signal[i] = Ciphertext("s_" +std::to_string(i));
    }

    for (int i = 0; i < signal_len ; i++) {
        Ciphertext acc = encrypt(0);

        for (int j = 0; j < filter_len ; j++) {
            if ((i - j) >= 0) {
                acc+= signal[i - j] * filter[j];
            }
        }
        output[i] = acc;
        output[i].set_output("output_" + std::to_string(i));

    }
}

void print_bool_arg(bool arg, const string &name, ostream &os)
{
  os << (arg ? name : "no_" + name);
}

int main(int argc, char **argv) {
    bool vectorized = true;
  if (argc > 1)
    vectorized = stoi(argv[1]);

  int window = 0;
  if (argc > 2) 
    window = stoi(argv[2]);

  bool call_quantifier = true;
  if (argc > 3)
    call_quantifier = stoi(argv[3]);

  bool cse = true;
  if (argc > 4)
    cse = stoi(argv[4]);
  
  int slot_count = 1 ;
  if (argc > 5)
    slot_count = stoi(argv[5]);

  bool const_folding = true;
  if (argc > 5)
    const_folding = stoi(argv[5]);

  if (cse)
  {
    Compiler::enable_cse();
    Compiler::enable_order_operands();
  }
  else
  {
    Compiler::disable_cse();
    Compiler::disable_order_operands();
  }

  if (const_folding)
    Compiler::enable_const_folding();
  else
    Compiler::disable_const_folding(); 

  chrono::high_resolution_clock::time_point t;
  chrono::duration<double, milli> elapsed;
  string func_name = "fhe";
  /**************/t = chrono::high_resolution_clock::now();

  if(vectorized) {
    int benchmark_type = STRUCTURED_WITH_MULTIPLE_OUTPUTS;
    const auto &func = Compiler::create_func(func_name, 1, 20, false, true);
      fhe(slot_count);
      string gen_name = "_gen_he_" + func_name;
      string gen_path = "he/" + gen_name;
      ofstream header_os(gen_path + ".hpp");
      if (!header_os)
        throw logic_error("failed to create header file");
      ofstream source_os(gen_path + ".cpp");
      if (!source_os)
        throw logic_error("failed to create source file");
      cout << " window is " << window << endl;
      Compiler::gen_vectorized_code(func, window, benchmark_type);
      if (SIMPLIFICATION_WITH_EGRAPHS) {
          Compiler_Simplification::compile(func, header_os, gen_name + ".hpp", source_os, true, 0);
      } else {
        //   Compiler::gen_he_code(func, header_os, gen_name + ".hpp", source_os);
          auto ruleset = Compiler::Ruleset::ops_cost;
          auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
          Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);
      }      /************/elapsed = chrono::high_resolution_clock::now() - t;
      cout << elapsed.count() << " ms\n";
      if (call_quantifier)
      {
        util::Quantifier quantifier{func};
        quantifier.run_all_analysis();
        quantifier.print_info(cout);
      }
  } else {
    // to add
  }

  return 0;

}