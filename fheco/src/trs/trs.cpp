#include "trs.hpp"
#include "draw_ir.hpp"
#include "ir_utils.hpp"
#include "test/dummy_ruleset.hpp"
#include "trs_const.hpp"
#include "trs_util_functions.hpp"
#include <variant>

namespace fheco_trs
{

core::FunctionTable TRS::functions_table = util_functions::functions_table;

size_t TRS::rewrites_count = 0;

void TRS::apply_rule_on_ir_node(
  const std::shared_ptr<ir::Term> &ir_node, const RewriteRule &rule, bool &is_rule_applied)
{

  is_rule_applied = false;

  auto matching_map = rule.match_with_ir_node(ir_node, program);

  if (matching_map != std::nullopt)
  {
    // std::cout << "matched ... \n";
    if (rule.get_rewrite_condition() != std::nullopt)
    {
      // std::cout << "checking condition ...\n";
      if (rule.evaluate_rewrite_condition(*matching_map, program, functions_table))
      {
        is_rule_applied = rule.substitute_in_ir(ir_node, *matching_map, program, functions_table);
        rewrites_count++;
      }
    }
    else
    {
      is_rule_applied = rule.substitute_in_ir(ir_node, *matching_map, program, functions_table);
      rewrites_count++;
    }
  }
}

void TRS::apply_rules_on_ir_node(const std::shared_ptr<ir::Term> &node, const std::vector<RewriteRule> &rules)
{
  size_t curr_rule_index = 0;
  while (curr_rule_index < rules.size())
  {
    /*
      new_nodes_matching_pairs supposed to contain matching pairs for constants only
    */
    bool was_rule_applied = false;
    auto &rule = rules[curr_rule_index];
    do
    {
      apply_rule_on_ir_node(node, rule, was_rule_applied);
    } while (was_rule_applied == true);

    curr_rule_index += 1;
  }
}

void TRS::apply_rewrite_rules_on_program_from_static_ruleset()
{

  auto &sorted_nodes = program->get_dataflow_sorted_nodes(true);
  for (auto &node : sorted_nodes)
  {
    if (node->get_opcode() == ir::OpCode::undefined)
      continue;

    switch (node->get_opcode())
    {
    case ir::OpCode::add:
      apply_rules_on_ir_node(node, ruleset::add_ruleset);
      break;

    case ir::OpCode::sub:
      apply_rules_on_ir_node(node, ruleset::sub_ruleset);
      break;

    case ir::OpCode::mul:
      apply_rules_on_ir_node(node, ruleset::mul_ruleset);
      break;

    case ir::OpCode::rotate:
      apply_rules_on_ir_node(node, ruleset::rotations_ruleset);
      break;

    default:
      break;
    }
  }
}

void TRS::apply_rewrite_rules_on_program(const std::vector<RewriteRule> &ruleset)
{
  auto &sorted_nodes = program->get_dataflow_sorted_nodes(true);
  for (auto &node : sorted_nodes)
  {
    if (node->get_opcode() == ir::OpCode::undefined)
      continue;

    apply_rules_on_ir_node(node, ruleset);
  }
}

} // namespace fheco_trs
