#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <random>
#include <algorithm> 
#include <sstream>
#include <fstream>   

using namespace std;


enum class NodeTypeV8 { INPUT, OPERATION };

struct DagNodeV8 {
    string id; 
    string c_style_var_name; 
    NodeTypeV8 type;
    string op_name;
    vector<string> inputs_ids; 
    long long literal_plaintext_val; 
    int depth_level;
    int fan_out;
    DagNodeV8() : literal_plaintext_val(0), depth_level(0), fan_out(0) {}
};

map<string, int> DEFAULT_OP_DISTRIBUTION_V8 = {
    {"Add", 35}, {"Mult", 35}, {"AddP", 15}, {"MultP", 15}
};

static mt19937 rng_v8_gen(random_device{}()); 

int getRandomIntV8Gen(int min_val, int max_val) { 
    if (min_val > max_val) swap(min_val, max_val); 
    if (min_val == max_val) return min_val; 
    uniform_int_distribution<int> dist(min_val, max_val); 
    return dist(rng_v8_gen); 
}
long long getRandomLongV8Gen(long long min_val, long long max_val) { 
    if (min_val > max_val) swap(min_val, max_val); 
    if (min_val == max_val) return min_val; 
    uniform_int_distribution<long long> dist(min_val, max_val); 
    return dist(rng_v8_gen); 
}
double getRandomProbV8Gen() { 
    uniform_real_distribution<double> dist(0.0, 1.0); 
    return dist(rng_v8_gen); 
}
string choose_weighted_random_op_v8_gen(const map<string, int>& distribution) { 
    if (distribution.empty()) return ""; 
    vector<string> ops; vector<int> weights; int total_weight = 0;
    for (const auto& pair_item : distribution) {
        ops.push_back(pair_item.first); weights.push_back(pair_item.second); total_weight += pair_item.second;
    }
    if (total_weight == 0) return ops.empty() ? "" : ops[getRandomIntV8Gen(0, ops.size() - 1)];
    uniform_int_distribution<int> dist(1, total_weight);
    int random_num = dist(rng_v8_gen); int current_sum = 0;
    for (size_t i = 0; i < ops.size(); ++i) {
        current_sum += weights[i]; if (random_num <= current_sum) return ops[i];
    }
    return "";
}

static int global_c_var_counter_v8_gen = 0; 
void reset_c_var_counter_v8_gen() { global_c_var_counter_v8_gen = 0; }
string get_next_c_style_var_name_v8_gen() { return "c_" + to_string(global_c_var_counter_v8_gen++); }


string generate_dag_internal_v8( 
    int num_initial_inputs,
    int target_depth,
    double p_reuse,
    int ops_per_level,
    map<string, DagNodeV8>& out_all_nodes,
    vector<string>& out_fhe_io_lines,
    vector<string>& out_generated_cpp_body_lines,
    const map<string, int>& op_distribution = DEFAULT_OP_DISTRIBUTION_V8,
    pair<long long, long long> plaintext_val_range = {1, 10}
) {
    out_all_nodes.clear(); out_fhe_io_lines.clear(); out_generated_cpp_body_lines.clear();
    reset_c_var_counter_v8_gen(); 

    string first_input_cpp_var_for_depth0_case;

    if (target_depth <= 0 || num_initial_inputs == 0) {
        if (target_depth <= 0 && num_initial_inputs > 0) {
            string node_id_internal = "InputNode_0"; 
            string c_style_name = get_next_c_style_var_name_v8_gen(); 
            first_input_cpp_var_for_depth0_case = c_style_name;
            DagNodeV8 input_node; input_node.id = node_id_internal; input_node.c_style_var_name = c_style_name;
            input_node.type = NodeTypeV8::INPUT; input_node.depth_level = 0;
            out_all_nodes[node_id_internal] = input_node;
            out_fhe_io_lines.push_back(c_style_name + " 1 0 " + to_string(getRandomIntV8Gen(0, 100)));
            // La DÉCLARATION sera faite par la fonction qui enveloppe
            return first_input_cpp_var_for_depth0_case; // Retourne le nom de la var c_X de la "sortie"
        }
        return ""; 
    }
    map<string, int> current_op_distribution = op_distribution.empty() ? DEFAULT_OP_DISTRIBUTION_V8 : op_distribution;
    if (current_op_distribution.empty()){
         if (num_initial_inputs > 0) {
            string node_id_internal = "InputNode_0"; string c_style_name = get_next_c_style_var_name_v8_gen();
            first_input_cpp_var_for_depth0_case = c_style_name;
            DagNodeV8 input_node; input_node.id = node_id_internal; input_node.c_style_var_name = c_style_name;
            input_node.type = NodeTypeV8::INPUT; input_node.depth_level = 0;
            out_all_nodes[node_id_internal] = input_node;
            out_fhe_io_lines.push_back(c_style_name + " 1 0 " + to_string(getRandomIntV8Gen(0, 100)));
            return first_input_cpp_var_for_depth0_case;
        } return "";
    }
    
    vector<string> initial_input_node_ids_internal;
    for (int i = 0; i < num_initial_inputs; ++i) {
        string node_id = "InputNode_" + to_string(i); 
        string c_style_name = get_next_c_style_var_name_v8_gen(); 
        DagNodeV8 input_node; input_node.id = node_id; input_node.c_style_var_name = c_style_name;
        input_node.type = NodeTypeV8::INPUT; input_node.depth_level = 0;
        out_all_nodes[node_id] = input_node; initial_input_node_ids_internal.push_back(node_id);
        out_fhe_io_lines.push_back(c_style_name + " 1 0 " + to_string(getRandomIntV8Gen(0, 100)));
    }

    vector<string> operands_from_prev_level_ids = initial_input_node_ids_internal;

    for (int current_d_level = 1; current_d_level < target_depth; ++current_d_level) {
        vector<string> nodes_created_this_level_ids;
        vector<string> reuse_pool_ids;
        for(const auto& entry : out_all_nodes) {
            if (entry.second.depth_level < current_d_level) reuse_pool_ids.push_back(entry.first);
        }
        shuffle(reuse_pool_ids.begin(), reuse_pool_ids.end(), rng_v8_gen);
        if (operands_from_prev_level_ids.empty() && reuse_pool_ids.empty()) break;
        
        int current_ops_this_level = ops_per_level;
        if (target_depth > 1 && ops_per_level == 0 && current_d_level == 1 && num_initial_inputs >=1) {
            if (num_initial_inputs >=1 ) current_ops_this_level = 1; 
        }

        for (int op_idx = 0; op_idx < current_ops_this_level; ++op_idx) {
            string chosen_op_name = choose_weighted_random_op_v8_gen(current_op_distribution);
            if (chosen_op_name.empty()) continue;
            
            vector<string> current_op_ct_inputs_node_ids_internal; 
            long long literal_pt_for_op = 0;
            bool possible_to_form_op = true;

            string op1_node_id_internal;
            vector<string> source_options_op1;
            if (getRandomProbV8Gen() < p_reuse && !reuse_pool_ids.empty()) source_options_op1.insert(source_options_op1.end(), reuse_pool_ids.begin(), reuse_pool_ids.end());
            if (!operands_from_prev_level_ids.empty()) source_options_op1.insert(source_options_op1.end(), operands_from_prev_level_ids.begin(), operands_from_prev_level_ids.end());
            if (source_options_op1.empty() && !initial_input_node_ids_internal.empty()) source_options_op1.insert(source_options_op1.end(), initial_input_node_ids_internal.begin(), initial_input_node_ids_internal.end());

            if (!source_options_op1.empty()) {
                set<string> unique_set1(source_options_op1.begin(), source_options_op1.end());
                vector<string> unique_vec1(unique_set1.begin(), unique_set1.end());
                op1_node_id_internal = unique_vec1[getRandomIntV8Gen(0, unique_vec1.size() - 1)];
                current_op_ct_inputs_node_ids_internal.push_back(op1_node_id_internal);
            } else { possible_to_form_op = false; }

            if (possible_to_form_op) {
                if (chosen_op_name == "AddP" || chosen_op_name == "MultP") {    
                    literal_pt_for_op = getRandomLongV8Gen(plaintext_val_range.first, plaintext_val_range.second);
                } else { 
                    string op2_node_id_internal;
                    vector<string> source_options_op2;
                    if (getRandomProbV8Gen() < p_reuse && !reuse_pool_ids.empty()) source_options_op2.insert(source_options_op2.end(), reuse_pool_ids.begin(), reuse_pool_ids.end());
                    if (!operands_from_prev_level_ids.empty()) source_options_op2.insert(source_options_op2.end(), operands_from_prev_level_ids.begin(), operands_from_prev_level_ids.end());
                    if (source_options_op2.empty() && !initial_input_node_ids_internal.empty()) source_options_op2.insert(source_options_op2.end(), initial_input_node_ids_internal.begin(), initial_input_node_ids_internal.end());
                    if (source_options_op2.empty() && !source_options_op1.empty()) source_options_op2 = source_options_op1;

                    if (!source_options_op2.empty()) {
                        set<string> unique_set2(source_options_op2.begin(), source_options_op2.end());
                        vector<string> unique_vec2(unique_set2.begin(), unique_set2.end());
                        op2_node_id_internal = unique_vec2[getRandomIntV8Gen(0, unique_vec2.size() - 1)];
                        current_op_ct_inputs_node_ids_internal.push_back(op2_node_id_internal);
                    } else { possible_to_form_op = false; }
                }
            }
            if (!possible_to_form_op) continue;

            string new_op_node_id_internal = "OpNode_" + to_string(current_d_level) + "_" + to_string(op_idx) + "_" + to_string(rng_v8_gen() % 10000);
            
            DagNodeV8 new_op_node;
            new_op_node.id = new_op_node_id_internal; 
            new_op_node.c_style_var_name = get_next_c_style_var_name_v8_gen(); 
            new_op_node.type = NodeTypeV8::OPERATION; 
            new_op_node.op_name = chosen_op_name;
            new_op_node.inputs_ids = current_op_ct_inputs_node_ids_internal;
            if (chosen_op_name == "AddP" || chosen_op_name == "MultP") {
                new_op_node.literal_plaintext_val = literal_pt_for_op;
            }
            new_op_node.depth_level = current_d_level;
            out_all_nodes[new_op_node.id] = new_op_node;
            nodes_created_this_level_ids.push_back(new_op_node.id);
            
            string cpp_line = "    Ciphertext " + new_op_node.c_style_var_name + " = "; 
            cpp_line += out_all_nodes.at(current_op_ct_inputs_node_ids_internal[0]).c_style_var_name;
            if (chosen_op_name == "Add") cpp_line += " + " + out_all_nodes.at(current_op_ct_inputs_node_ids_internal[1]).c_style_var_name;
            else if (chosen_op_name == "Mult") cpp_line += " * " + out_all_nodes.at(current_op_ct_inputs_node_ids_internal[1]).c_style_var_name;
            else if (chosen_op_name == "AddP") cpp_line += " + " + to_string(literal_pt_for_op);
            else if (chosen_op_name == "MultP") cpp_line += " * " + to_string(literal_pt_for_op);
            cpp_line += ";";
            out_generated_cpp_body_lines.push_back(cpp_line); 
            for (const string& inp_id : current_op_ct_inputs_node_ids_internal) {
                out_all_nodes.at(inp_id).fan_out++;
            }
        }
        if (nodes_created_this_level_ids.empty() && current_d_level < target_depth -1) break;
        operands_from_prev_level_ids = nodes_created_this_level_ids;
    }

    string final_output_c_var_name = "output"; 
    
    vector<string> final_op_candidates_ids_internal = operands_from_prev_level_ids;
    if (final_op_candidates_ids_internal.empty()) final_op_candidates_ids_internal = initial_input_node_ids_internal;
    if (final_op_candidates_ids_internal.empty()) return "";

    set<string> unique_final_set(final_op_candidates_ids_internal.begin(), final_op_candidates_ids_internal.end());
    vector<string> unique_final_ops_candidates_internal(unique_final_set.begin(), unique_final_set.end());
    
    string final_op_name_cpp;
    if (current_op_distribution.empty() || unique_final_ops_candidates_internal.empty()) {
        string chosen_final_candidate_id = final_op_candidates_ids_internal[0];
        out_generated_cpp_body_lines.push_back("    Ciphertext " + final_output_c_var_name + " = " + out_all_nodes.at(chosen_final_candidate_id).c_style_var_name + ";");
        return final_output_c_var_name;
    }
    final_op_name_cpp = choose_weighted_random_op_v8_gen(current_op_distribution);
     if(final_op_name_cpp.empty()){
        string chosen_final_candidate_id = final_op_candidates_ids_internal[0];
        out_generated_cpp_body_lines.push_back("    Ciphertext " + final_output_c_var_name + " = " + out_all_nodes.at(chosen_final_candidate_id).c_style_var_name + ";");
        return final_output_c_var_name;
    }

    vector<string> final_op_ct_inputs_node_ids_internal; long long final_literal_pt = 0;
    string final_op1_id_internal = unique_final_ops_candidates_internal[getRandomIntV8Gen(0, unique_final_ops_candidates_internal.size()-1)];
    final_op_ct_inputs_node_ids_internal.push_back(final_op1_id_internal);

    if (final_op_name_cpp == "AddP" || final_op_name_cpp == "MultP") {
        final_literal_pt = getRandomLongV8Gen(plaintext_val_range.first, plaintext_val_range.second);
    } else { 
        string final_op2_id_internal;
        if (unique_final_ops_candidates_internal.size() >= 2) { 
            vector<string> temp_choices = unique_final_ops_candidates_internal;
            temp_choices.erase(remove(temp_choices.begin(), temp_choices.end(), final_op1_id_internal), temp_choices.end());
            if (!temp_choices.empty()) { final_op2_id_internal = temp_choices[getRandomIntV8Gen(0, temp_choices.size() -1)]; } 
            else { final_op2_id_internal = final_op1_id_internal; }
        } else { final_op2_id_internal = final_op1_id_internal; }
        final_op_ct_inputs_node_ids_internal.push_back(final_op2_id_internal);
    }
    
    string final_op_node_id_internal = "OpNode_final_" + to_string(target_depth) + "_" + to_string(rng_v8_gen()%1000);
    DagNodeV8 final_op_node;
    final_op_node.id = final_op_node_id_internal; 
    final_op_node.c_style_var_name = final_output_c_var_name; 
    final_op_node.type = NodeTypeV8::OPERATION; 
    final_op_node.op_name = final_op_name_cpp;
    final_op_node.inputs_ids = final_op_ct_inputs_node_ids_internal;
    if (final_op_name_cpp == "AddP" || final_op_name_cpp == "MultP") {
        final_op_node.literal_plaintext_val = final_literal_pt;
    }
    final_op_node.depth_level = target_depth;
    out_all_nodes[final_op_node.id] = final_op_node; 

    string cpp_line_final = "    Ciphertext " + final_output_c_var_name + " = "; 
    cpp_line_final += out_all_nodes.at(final_op_ct_inputs_node_ids_internal[0]).c_style_var_name;
    if (final_op_name_cpp == "Add") cpp_line_final += " + " + out_all_nodes.at(final_op_ct_inputs_node_ids_internal[1]).c_style_var_name;
    else if (final_op_name_cpp == "Mult") cpp_line_final += " * " + out_all_nodes.at(final_op_ct_inputs_node_ids_internal[1]).c_style_var_name;
    else if (final_op_name_cpp == "AddP") cpp_line_final += " + " + to_string(final_literal_pt);
    else if (final_op_name_cpp == "MultP") cpp_line_final += " * " + to_string(final_literal_pt);
    cpp_line_final += ";";
    out_generated_cpp_body_lines.push_back(cpp_line_final);
    for (const string& inp_id : final_op_ct_inputs_node_ids_internal) {
        out_all_nodes.at(inp_id).fan_out++;
    }
    
    return final_output_c_var_name; 
}

struct DagParamsV8 { 
    int num_initial_inputs;
    int target_depth;
    double p_reuse;
    int ops_per_level;
    map<string, int> op_distribution;
    pair<long long, long long> plaintext_val_range;
};

DagParamsV8 F_F_params_v8() {  return {getRandomIntV8Gen(2,3),getRandomIntV8Gen(1,3),0.15,getRandomIntV8Gen(1,2),{{"Add",50},{"Mult",20},{"AddP",15},{"MultP",15}},{1,10}}; }
DagParamsV8 F_E_params_v8() {  return {getRandomIntV8Gen(2,4),getRandomIntV8Gen(2,4),0.75,getRandomIntV8Gen(2,4),DEFAULT_OP_DISTRIBUTION_V8,{1,10}}; }
DagParamsV8 E_F_params_v8() {  return {getRandomIntV8Gen(2,3),getRandomIntV8Gen(8,12),0.2,getRandomIntV8Gen(1,3),DEFAULT_OP_DISTRIBUTION_V8,{1,10}}; }
DagParamsV8 E_E_params_v8() {  return {getRandomIntV8Gen(3,5),getRandomIntV8Gen(8,15),0.7,getRandomIntV8Gen(2,5),DEFAULT_OP_DISTRIBUTION_V8,{1,10}}; }


void write_fhe_dag_program_file(
    const string& output_cpp_filename,
    const string& output_io_filename,  
    const DagParamsV8& params,
    const string& fhe_dag_function_name = "fhe_dag_benchmark"
) {
    map<string, DagNodeV8> all_dag_nodes_metadata;
    vector<string> io_file_lines;
    vector<string> cpp_dag_body_lines; 

    string final_dag_output_c_var = generate_dag_internal_v8(
        params.num_initial_inputs,
        params.target_depth,
        params.p_reuse,
        params.ops_per_level,
        all_dag_nodes_metadata, 
        io_file_lines,          
        cpp_dag_body_lines,         
        params.op_distribution,
        params.plaintext_val_range
    );

    ofstream cpp_out_file(output_cpp_filename);
    if (!cpp_out_file.is_open()) {
        cerr << "Error: Could not open " << output_cpp_filename << " for writing." << endl;
        return;
    }

    cpp_out_file << "#include \"fheco/fheco.hpp\"" << endl;
    cpp_out_file << endl;
    cpp_out_file << "using namespace std;" << endl;
    cpp_out_file << "using namespace fheco;" << endl;
    cpp_out_file << "#include <chrono>" << endl; 
    cpp_out_file << "#include <fstream>" << endl; 
    cpp_out_file << "#include <iostream>" << endl;
    cpp_out_file << "#include <string>" << endl;
    cpp_out_file << "#include <vector>" << endl;
   
    cpp_out_file << endl;

    cpp_out_file << "// --- FHE DAG Benchmark Function ---" << endl;
    cpp_out_file << "void " << fhe_dag_function_name << "(int slot_count_param) {" << endl; // Ajout de slot_count_param si nécessaire
    cpp_out_file << "    // (slot_count_param is passed but not directly used in this generated DAG body)" << endl;
    
    for (int i = 0; i < params.num_initial_inputs; ++i) {
        string c_style_name_for_input;
        
        c_style_name_for_input = "c_" + to_string(i); 
        
        cpp_out_file << "    Ciphertext " << c_style_name_for_input 
                     << "(\"" << c_style_name_for_input << "\");" << endl;
    }
    cpp_out_file << endl;

    for (const string& line : cpp_dag_body_lines) {
        cpp_out_file << line << endl;
    }

    if (!final_dag_output_c_var.empty()) {
       
        if (final_dag_output_c_var != "output" && !cpp_dag_body_lines.empty()) {
            
            if (all_dag_nodes_metadata.at(final_dag_output_c_var).type == NodeTypeV8::INPUT && cpp_dag_body_lines.empty()){
                cpp_out_file << "    Ciphertext output = " << final_dag_output_c_var << ";" << endl;
            }
        }
      
        bool set_output_added = false;
        for(const string& l : cpp_dag_body_lines) { if (l.find(".set_output") != string::npos) {set_output_added = true; break;} }
        
        if (!set_output_added) {
             cpp_out_file << "    output.set_output(\"result\"); // Fallback set_output" << endl;
        }

    } else {
        cpp_out_file << "    Ciphertext output; // Dummy output" << endl;
        cpp_out_file << "    output.set_output(\"result\");" << endl;
    }

    cpp_out_file << "}" << endl << endl; 

    cpp_out_file << "void print_bool_arg(bool arg, const string &name, ostream &os)" << endl;
    cpp_out_file << "{" << endl;
    cpp_out_file << "  os << (arg ? name : \"no_\" + name);" << endl;
    cpp_out_file << "}" << endl << endl;

    cpp_out_file << "// --- Main Function (adapted from example) ---" << endl;
    cpp_out_file << "int main(int argc, char **argv)" << endl;
    cpp_out_file << "{" << endl;
    cpp_out_file << "  bool vectorized = true;" << endl;
    cpp_out_file << "  if (argc > 1) vectorized = stoi(argv[1]);" << endl << endl;
    cpp_out_file << "  int window = 0;" << endl;
    cpp_out_file << "  if (argc > 2) window = stoi(argv[2]);" << endl << endl;
    cpp_out_file << "  bool call_quantifier = true;" << endl;
    cpp_out_file << "  if (argc > 3) call_quantifier = stoi(argv[3]);" << endl << endl;
    cpp_out_file << "  bool cse = true;" << endl;
    cpp_out_file << "  if (argc > 4) cse = stoi(argv[4]);" << endl << endl;
    cpp_out_file << "  int slot_count = 1 ; // Default, can be an arg" << endl;
    cpp_out_file << "  if (argc > 5) slot_count = stoi(argv[5]);" << endl << endl;
    cpp_out_file << "  bool const_folding = true;" << endl;
    cpp_out_file << "  if (argc > 6) const_folding = stoi(argv[6]);" << endl << endl;

    cpp_out_file << "  if (cse)" << endl;
    cpp_out_file << "  {" << endl;
    cpp_out_file << "    Compiler::enable_cse();" << endl;
    cpp_out_file << "    Compiler::enable_order_operands();" << endl;
    cpp_out_file << "  }" << endl;
    cpp_out_file << "  else" << endl;
    cpp_out_file << "  {" << endl;
    cpp_out_file << "    Compiler::disable_cse();" << endl;
    cpp_out_file << "    Compiler::disable_order_operands();" << endl;
    cpp_out_file << "  }" << endl << endl;

    cpp_out_file << "  if (const_folding)" << endl;
    cpp_out_file << "    Compiler::enable_const_folding();" << endl;
    cpp_out_file << "  else" << endl;
    cpp_out_file << "    Compiler::disable_const_folding();" << endl << endl;

    cpp_out_file << "  chrono::high_resolution_clock::time_point t;" << endl;
    cpp_out_file << "  chrono::duration<double, milli> elapsed;" << endl;
    cpp_out_file << "  string func_name = \"" << fhe_dag_function_name << "\";" << endl;
    cpp_out_file << "  /**************/t = chrono::high_resolution_clock::now();" << endl;
    
  
    cpp_out_file << "  if (vectorized)" << endl;
    cpp_out_file << "  { " << endl;
   
    cpp_out_file << "      int benchmark_type = 0; // Placeholder pour UNSTRUCTURED_WITH_ONE_OUTPUT " << endl;
    cpp_out_file << "      cout << \"Using benchmark_type: \" << benchmark_type << endl;" << endl;


    cpp_out_file << "      const auto &func = Compiler::create_func(func_name, slot_count, params.target_depth + 5, true, true);" << endl;
    cpp_out_file << "      " << fhe_dag_function_name << "(slot_count); // Appel de notre fonction DAG générée" << endl;
    
    cpp_out_file << "      string gen_name = \"_gen_he_\" + func_name;" << endl;
    cpp_out_file << "      string gen_path = \"he/\" + gen_name;" << endl; 
    cpp_out_file << "      ofstream header_os(gen_path + \".hpp\");" << endl;
    cpp_out_file << "      if (!header_os) throw logic_error(\"failed to create header file \" + gen_path + \".hpp\");" << endl;
    cpp_out_file << "      ofstream source_os(gen_path + \".cpp\");" << endl;
    cpp_out_file << "      if (!source_os) throw logic_error(\"failed to create source file \" + gen_path + \".cpp\");" << endl;
    cpp_out_file << "      cout << \" window is \" << window << endl;" << endl;
    cpp_out_file << "      Compiler::gen_vectorized_code(func, window, benchmark_type);" << endl;
      
  

    cpp_out_file << "      auto ruleset = Compiler::Ruleset::ops_cost;" << endl;
    cpp_out_file << "      auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;" << endl;
    cpp_out_file << "      Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + \".hpp\", source_os);" << endl;


    cpp_out_file << "      /************/elapsed = chrono::high_resolution_clock::now() - t;" << endl;   
    cpp_out_file << "      cout << elapsed.count() << \" ms\\n\";" << endl;
    cpp_out_file << "      if (call_quantifier)" << endl;
    cpp_out_file << "      {" << endl;
    cpp_out_file << "        util::Quantifier quantifier{func};" << endl;
    cpp_out_file << "        quantifier.run_all_analysis();" << endl;
    cpp_out_file << "        quantifier.print_info(cout);" << endl;
    cpp_out_file << "      }" << endl;
    cpp_out_file << "  }" << endl; 
    cpp_out_file << "  else { cout << \"Non-vectorized path not implemented in this generated main.\" << endl; }" << endl;


    cpp_out_file << "  return 0;" << endl;
    cpp_out_file << "}" << endl; 
    cpp_out_file.close();
    cout << "Generated FHE program with main: " << output_cpp_filename << endl;


    ofstream io_out_file(output_io_filename, std::ios::trunc);
    if (io_out_file.is_open()) {
        if (!io_file_lines.empty()) {
            io_out_file << "1 " << io_file_lines.size() << " 1" << endl; 
            for (const string& line : io_file_lines) {
                io_out_file << line << endl;
            }
        } else { 
            io_out_file << "1 0 1" << endl; 
        }
        io_out_file.close();
        cout << "Generated IO definition file: " << output_io_filename << endl;
    } else {
        cerr << "Error: Could not open " << output_io_filename << " for writing." << endl;
    }
}

int main(int argc, char **argv) { 
    int num_inputs_arg = 3;
    int target_depth_arg = 5;
    double p_reuse_arg = 0.5;
    int ops_per_level_arg = 2;
    string benchmark_config = "F_E"; 

    if (argc > 1) benchmark_config = argv[1]; 
    if (argc > 2) num_inputs_arg = stoi(argv[2]);
    if (argc > 3) target_depth_arg = stoi(argv[3]);
    if (argc > 4) p_reuse_arg = stod(argv[4]);
    if (argc > 5) ops_per_level_arg = stoi(argv[5]);
    
    DagParamsV8 params;
    if (benchmark_config == "F_F") params = F_F_params_v8();
    else if (benchmark_config == "F_E") params = F_E_params_v8();
    else if (benchmark_config == "E_F") params = E_F_params_v8();
    else if (benchmark_config == "E_E") params = E_E_params_v8();
    else { 
        cout << "Using custom/default parameters based on CLI args or built-in defaults." << endl;
        params.num_initial_inputs = num_inputs_arg;
        params.target_depth = target_depth_arg;
        params.p_reuse = p_reuse_arg;
        params.ops_per_level = ops_per_level_arg;
        params.op_distribution = DEFAULT_OP_DISTRIBUTION_V8; 
        params.plaintext_val_range = {1, 10};
    }


    cout << "Generating FHE DAG program with chosen/parsed params: " << endl;
    cout << "  Config: " << benchmark_config
         << ", Inputs: " << params.num_initial_inputs 
         << ", Target Depth: " << params.target_depth
         << ", p_reuse: " << params.p_reuse 
         << ", ops/level: " << params.ops_per_level << endl;

    string output_cpp_filename = "fhe_generated_dag.cpp"; 
    string output_io_filename = "fhe_io_example_dag.txt";  
    string fhe_func_name_in_file = "fhe_dag_benchmark"; 

    write_fhe_dag_program_file(
        output_cpp_filename,
        output_io_filename,
        params,
        fhe_func_name_in_file
    );
    
    cout << "\nTo compile the generated DAG with FHECO, you would typically do:" << endl;
    cout << "1. Create a main_for_dag.cpp that includes \"" << output_cpp_filename << "\" (or its header) and calls " << fhe_func_name_in_file << "()." << endl;
    cout << "2. Ensure fhe_io_example_dag.txt is present." << endl;
    cout << "3. Compile main_for_dag.cpp with your FHECO library and flags." << endl;
    cout << "   Example (conceptual):" << endl;
    cout << "   g++ -std=c++17 main_for_dag.cpp " << output_cpp_filename << " $(FHECO_FLAGS) $(FHECO_LIBS) -o compiled_dag_benchmark" << endl;


    return 0;
}