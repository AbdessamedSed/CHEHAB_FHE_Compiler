#include "fheco/fheco.hpp"

using namespace std;
using namespace fheco;
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector> 
/********************/
void fhe_vectorized(int slot_count){
  Ciphertext c1("c1"); 
  Ciphertext c2("c2");
  Ciphertext slot_wise_diff = (c1 - c2) * (c1 - c2);
  Ciphertext sum = SumVec(slot_wise_diff,slot_count);
  sum.set_output("result"); 
}
/***************************/
// void fhe(int slot_count)
// {
//   size_t size = slot_count;
//   std::vector<Ciphertext> v1(size);
//   std::vector<Ciphertext> v2(size);
//   //std::vector<Ciphertext> output(size);
//   Ciphertext output = Ciphertext(Plaintext(0));
//   for (int i = 0; i < size; i++)
//   {
//     v1[i] = Ciphertext("c1_" + std::to_string(i));
//   }
//   for (int i = 0; i < size; i++)
//   { 
//     v2[i] = Ciphertext("c2_" + std::to_string(i));
//     if(i==0){
//         output = (v2[i] - v1[i]) * (v2[i] - v1[i]); 
//     }else{
//       output += (v2[i] - v1[i]) * (v2[i] - v1[i]);
//     }
//   }
//   output.set_output("result");
// }

// void fhe(int slot_count)
// {

//     Ciphertext output;

//     output.set_output("output");
// }

void fhe(int slot_count) {
  Ciphertext output;
  // output = (Ciphertext("v_46") + Ciphertext("v_234")) + (Ciphertext("v_1") + Ciphertext("v_93")) + ( Ciphertext("v_12") * Ciphertext("v_11") );
  // output = ( ( ( ( ( Ciphertext("v_739") * Ciphertext("v_26") ) + ( Ciphertext("v_405") + Ciphertext("v_446") ) ) * ( ( Ciphertext("v_991") * Ciphertext("v_341") ) * ( Ciphertext("v_125") * Ciphertext("v_50") ) ) ) * ( ( ( Ciphertext("v_294") + Ciphertext("v_673") ) * ( Ciphertext("v_474") * Ciphertext("v_210") ) ) * ( ( Ciphertext("v_33") * Ciphertext("v_787") ) * ( Ciphertext("v_309") + Ciphertext("v_511") ) ) ) ) + ( ( ( ( Ciphertext("v_3") * Ciphertext("v_276") ) + ( Ciphertext("v_404") * Ciphertext("v_152") ) ) * ( ( Ciphertext("v_16") + Ciphertext("v_848") ) + ( Ciphertext("v_115") + Ciphertext("v_620") ) ) ) + ( ( ( Ciphertext("v_623") + Ciphertext("v_234") ) * ( Ciphertext("v_42") * Ciphertext("v_595") ) ) + ( ( Ciphertext("v_421") + Ciphertext("v_484") ) + ( Ciphertext("v_305") + Ciphertext("v_718") ) ) ) ) );
  output = ( Ciphertext("v_46") + ( Ciphertext("v_234") + ( Ciphertext("v_405") + ( Ciphertext("v_446") + ( Ciphertext("v_991") + Ciphertext("v_341") ) ) ) ) );

  output.set_output("output");
}
/********************************************/
void print_bool_arg(bool arg, const string &name, ostream &os)
{
  os << (arg ? name : "no_" + name);
}
int main(int argc, char **argv)
{
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
  if (vectorized)
  { 
      const auto &func = Compiler::create_func(func_name, 1, 20, true, true);
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
      Compiler::gen_vectorized_code(func, window);
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
  }
  else
  {
      const auto &func = Compiler::create_func(func_name, slot_count, 20, true, true);
      fhe_vectorized(slot_count);
      string gen_name = "_gen_he_" + func_name;
      string gen_path = "he/" + gen_name;
      ofstream header_os(gen_path + ".hpp");
      if (!header_os)
        throw logic_error("failed to create header file");
      ofstream source_os(gen_path + ".cpp");
      if (!source_os)
        throw logic_error("failed to create source file");
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