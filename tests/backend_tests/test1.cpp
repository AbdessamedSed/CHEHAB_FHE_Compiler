#include "../build/frontend_tests/out/test1.hpp"
#include "seal/seal.h"
#include "ufhe/ufhe.hpp"
#include <bits/stdc++.h>

using namespace std;

void ufhe_version_test1()
{

  ufhe::Scheme scheme(ufhe::api::scheme_type::bfv); // just an enum, doesn't need to be an object
  ufhe::EncryptionParams params(scheme);
  size_t poly_modulus_degree = 8192;
  params.set_poly_modulus_degree(poly_modulus_degree);
  // this needs to be changed, it is more practical to work with bit lengths
  ufhe::CoeffModulus coeff_modulus({8796092858369, 8796092792833, 17592186028033, 17592185438209, 17592184717313});
  params.set_coeff_modulus(coeff_modulus);
  params.set_plain_modulus(ufhe::Modulus(1032193));

  /*
    for the modulus we can either keep them as object but we allow implicit conversions, otherwise we can just use
    uint64_t or size_t directly
  */

  /*
    for parameters something like the following will be good

    ufhe::EncryptionParams params(ufhe::scheme_type::bfv);
    params.set_poly_modulus_degree(4096);
    params.set_coeff_modulus(ufhe::CoeffModulus::BFVDefault(4096, ufhe::sec_level_type::tc128));
    params.set_plain_modulus(786433);
    ufhe::EncryptionContext context(params);

  */

  std::unordered_map<std::string, ufhe::Ciphertext> encrypted_inputs;
  std::unordered_map<std::string, ufhe::Plaintext> encoded_inputs;
  std::unordered_map<std::string, ufhe::Ciphertext> encrypted_outputs;
  std::unordered_map<std::string, ufhe::Plaintext> encoded_outputs;

  ufhe::EncryptionContext context(params);

  ufhe::KeyGenerator key_gen(context);
  ufhe::RelinKeys relin_keys;
  key_gen.create_relin_keys(relin_keys);
  ufhe::GaloisKeys galois_keys;
  ufhe::PublicKey public_key;
  key_gen.create_public_key(public_key);

  // test1(
  // encrypted_inputs, encoded_inputs, encrypted_outputs, encoded_outputs, context, relin_keys, galois_keys,
  // public_key);

  ufhe::SecretKey secret_key = key_gen.secret_key();

  ufhe::Decryptor decryptor(context, secret_key);

  ufhe::Plaintext decrypted_output;
  decryptor.decrypt(encrypted_outputs["sum_output"], decrypted_output);

  std::vector<int64_t> decoded_output;

  ufhe::BatchEncoder encoder(context);
  encoder.decode(decrypted_output, decoded_output);

  for (size_t i = 0; i < 10; i++)
  {
    std::cout << decoded_output[i] << " ";
  }
  std::cout << "\n";
}

void seal_version_test1()
{
  seal::EncryptionParameters params(seal::scheme_type::bfv);
  size_t poly_modulus_degree = 8192;
  params.set_poly_modulus_degree(poly_modulus_degree);
  params.set_coeff_modulus({8796092858369, 8796092792833, 17592186028033, 17592185438209, 17592184717313});
  params.set_plain_modulus(1032193);
  seal::SEALContext context(params);

  seal::KeyGenerator key_gen(context);
  seal::RelinKeys relin_keys;
  key_gen.create_relin_keys(relin_keys);
  seal::GaloisKeys galois_keys;
  seal::PublicKey public_key;
  key_gen.create_public_key(public_key);

  seal::BatchEncoder encoder(context);
  std::vector<int64_t> plaintext1_clear = {2, 3, 4, 5, 6, 7};
  seal::Plaintext plaintext1;
  encoder.encode(plaintext1_clear, plaintext1);
  seal::Encryptor encryptor(context, public_key);
  seal::Ciphertext ciphertext1;
  encryptor.encrypt(plaintext1, ciphertext1);
  std::vector<int64_t> plaintext0_clear = {1, 2, 3, 4, 5, 6};
  seal::Plaintext plaintext0;
  encoder.encode(plaintext0_clear, plaintext0);
  seal::Ciphertext ciphertext0;
  encryptor.encrypt(plaintext0, ciphertext0);
  seal::Evaluator evaluator(context);
  evaluator.add_inplace(ciphertext0, ciphertext1);
  seal::Ciphertext sum_output = ciphertext0;

  seal::SecretKey secret_key = key_gen.secret_key();

  seal::Decryptor decryptor(context, secret_key);

  seal::Plaintext decrypted_output;
  decryptor.decrypt(sum_output, decrypted_output);

  std::vector<int64_t> decoded_output;

  encoder.decode(decrypted_output, decoded_output);

  for (size_t i = 0; i < 10; i++)
  {
    std::cout << decoded_output[i] << " ";
  }
  std::cout << "\n";
}

int test1_backend()
{

  chrono::high_resolution_clock::time_point time_start, time_end;
  time_start = chrono::high_resolution_clock::now();
  ufhe_version_test1();
  time_end = chrono::high_resolution_clock::now();
  chrono::milliseconds time_diff = chrono::duration_cast<chrono::milliseconds>(time_end - time_start);
  std::cout << "ufhe time : " << time_diff.count() << "\n";

  time_start = chrono::high_resolution_clock::now();
  seal_version_test1();
  time_end = chrono::high_resolution_clock::now();
  time_diff = chrono::duration_cast<chrono::milliseconds>(time_end - time_start);
  std::cout << "seal time : " << time_diff.count() << "\n";

  return 0;
}
