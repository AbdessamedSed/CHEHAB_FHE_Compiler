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

using namespace std;
using namespace fheco;

void sobel(size_t width)
{
  Ciphertext img("img");
  Ciphertext top_row = img >> width;
  Ciphertext bottom_row = img << width;
  // gx
  vector<vector<integer>> gx_kernel = {{-1, 0, 1}, {-2, 0, 2}, {-1, 0, 1}};
  Ciphertext gx_top_sum =
    gx_kernel[0][0] * (top_row >> 1) + gx_kernel[0][1] * top_row + gx_kernel[0][2] * (top_row << 1);
  Ciphertext gx_curr_sum = gx_kernel[1][0] * (img >> 1) + gx_kernel[1][1] * img + gx_kernel[1][2] * (img << 1);
  Ciphertext gx_bottom_sum =
    gx_kernel[2][0] * (bottom_row >> 1) + gx_kernel[2][1] * bottom_row + gx_kernel[2][2] * (bottom_row << 1);
  Ciphertext gx_result = gx_top_sum + gx_curr_sum + gx_bottom_sum;
  // gy
  vector<vector<integer>> gy_kernel = {{-1, -2, -1}, {0, 0, 0}, {1, 2, 1}};
  Ciphertext gy_top_sum =
    gy_kernel[0][0] * (top_row >> 1) + gy_kernel[0][1] * top_row + gy_kernel[0][2] * (top_row << 1);
  Ciphertext gy_curr_sum = gy_kernel[1][0] * (img >> 1) + gy_kernel[1][1] * img + gy_kernel[1][2] * (img << 1);
  Ciphertext gy_bottom_sum =
    gy_kernel[2][0] * (bottom_row >> 1) + gy_kernel[2][1] * bottom_row + gy_kernel[2][2] * (bottom_row << 1);
  Ciphertext gy_result = gy_top_sum + gy_curr_sum + gy_bottom_sum;
  // combine
  Ciphertext result = gx_result * gx_result + gy_result * gy_result;
  result.set_output("result");
}

void print_bool_arg(bool arg, const string &name, ostream &os)
{
  os << (arg ? name : "no_" + name);
}

int main(int argc, char **argv)
{
  bool call_quantifier = false;
  if (argc > 1)
    call_quantifier = stoi(argv[1]);

  auto ruleset = Compiler::Ruleset::joined;
  if (argc > 2)
    ruleset = static_cast<Compiler::Ruleset>(stoi(argv[2]));

  auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
  if (argc > 3)
    rewrite_heuristic = static_cast<trs::RewriteHeuristic>(stoi(argv[3]));

  bool cse = true;
  if (argc > 4)
    cse = stoi(argv[4]);

  bool const_folding = true;
  if (argc > 5)
    const_folding = stoi(argv[5]);

  print_bool_arg(call_quantifier, "quantifier", clog);
  clog << " ";
  clog << ruleset << "_trs";
  clog << " ";
  clog << rewrite_heuristic;
  clog << " ";
  print_bool_arg(cse, "cse", clog);
  clog << " ";
  print_bool_arg(const_folding, "constant_folding", clog);
  clog << '\n';

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
  t = chrono::high_resolution_clock::now();
  string func_name = "sobel";
  size_t width = 64;
  size_t height = 64;
  const auto &func = Compiler::create_func(func_name, width * height, 20, true, true);
  sobel(width);

  string gen_name = "_gen_he_" + func_name;
  string gen_path = "he/" + gen_name;
  ofstream header_os(gen_path + ".hpp");
  if (!header_os)
    throw logic_error("failed to create header file");

  ofstream source_os(gen_path + ".cpp");
  if (!source_os)
    throw logic_error("failed to create source file");

  Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);
  elapsed = chrono::high_resolution_clock::now() - t;
  cout << elapsed.count() << " ms\n";

  if (call_quantifier)
  {
    util::Quantifier quantifier{func};
    quantifier.run_all_analysis();
    quantifier.print_info(cout);
  }
  return 0;
}
