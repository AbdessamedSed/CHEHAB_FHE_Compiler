#include "fhecompiler/fhecompiler.hpp"
#include "frontend_tests_const.hpp"
#include <bits/stdc++.h>

int test16_frontend()
{

  try
  {
    fhecompiler::init("test16", fhecompiler::Scheme::bfv, fhecompiler::Backend::SEAL);

    fhecompiler::Ciphertext ct1("ct1", fhecompiler::VarType::input);

    fhecompiler::Ciphertext ct2("ct2", fhecompiler::VarType::input);

    fhecompiler::Ciphertext ct1_output("ct1_output", fhecompiler::VarType::output);

    ct1.rotate_columns();

    ct1.square();

    ct2 += ct1;

    ct2.square();

    ct2.rotate_columns();

    ct2 -= ct1;

    ct1_output = ct2 - ct1;

    size_t polynomial_modulus_degree = 4096;
    size_t plaintext_modulus = 786433;

    params_selector::EncryptionParameters params;
    params.set_plaintext_modulus(plaintext_modulus);
    params.set_polynomial_modulus_degree(polynomial_modulus_degree);

    fhecompiler::compile(out_dir + "/" + "test16.hpp", &params);
  }
  catch (const char *emessage)
  {
    std::cout << emessage << '\n';
  }

  return 0;
}
