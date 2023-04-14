#include "matching_term.hpp"
#include "trs_const.hpp"

namespace fheco_trs
{
size_t MatchingTerm::term_id = 0;

MatchingTerm::MatchingTerm(uint64_t _value) : term_type(fheco_trs::TermType::scalar), value(_value), id(term_id++) {}
MatchingTerm::MatchingTerm(int64_t _value) : term_type(TermType::scalar), value(_value), id(term_id++) {}
MatchingTerm::MatchingTerm(int _value) : term_type(TermType::scalar), value(static_cast<int64_t>(_value)), id(term_id++) {}

MatchingTerm::MatchingTerm(bool _value) : term_type(TermType::boolean), value(static_cast<int64_t>(_value)), id(term_id++)
{}

MatchingTerm::MatchingTerm(FunctionId func_id) : term_type(fheco_trs::TermType::function), function_id(func_id) {}

MatchingTerm::MatchingTerm(
  fheco_trs::OpCode _opcode, const std::vector<MatchingTerm> &_operands, fheco_trs::TermType _term_type)
  : opcode(_opcode), id(term_id++), operands(_operands), term_type(_term_type)
{}

MatchingTerm::MatchingTerm(fheco_trs::TermType _term_type) : term_type(_term_type), id(term_id++) {}

MatchingTerm::MatchingTerm(const std::string &_label, fheco_trs::TermType _term_type)
  : label(_label), term_type(_term_type), id(term_id++)
{}

fheco_trs::TermType MatchingTerm::deduce_term_type(fheco_trs::TermType lhs_term_type, fheco_trs::TermType rhs_term_type)
{

  // Order of if statments is important !!!

  if (lhs_term_type == rhs_term_type)
    return lhs_term_type;

  else if (lhs_term_type == fheco_trs::TermType::variable || rhs_term_type == fheco_trs::TermType::variable)
    return fheco_trs::TermType::variable;

  else if (lhs_term_type == fheco_trs::TermType::ciphertext || rhs_term_type == fheco_trs::TermType::ciphertext)
    return fheco_trs::TermType::ciphertext;

  else if (lhs_term_type == fheco_trs::TermType::plaintext || rhs_term_type == fheco_trs::TermType::plaintext)
    return fheco_trs::TermType::plaintext;

  else if (lhs_term_type == fheco_trs::TermType::constant || rhs_term_type == fheco_trs::TermType::constant)
    return fheco_trs::TermType::constant;

  else
    throw("cannot deduce term type");
}

MatchingTerm MatchingTerm::flatten(MatchingTerm term)
{
  if (term.get_opcode() == fheco_trs::OpCode::undefined)
    return term;

  else
  {
    std::vector<MatchingTerm> new_operands;
    auto operands = term.get_operands();
    for (auto &operand : operands)
    {
      if (operand.get_opcode() == fheco_trs::OpCode::undefined)
        new_operands.push_back(operand);
      else if (operand.get_opcode() == term.get_opcode())
      {
        MatchingTerm flattened_operand = flatten(operand);
        auto sub_operands = flattened_operand.get_operands();

        for (auto &sub_operand : sub_operands)
          new_operands.push_back(sub_operand);
      }
    }
    term.set_operands(new_operands);
  }

  return term;
}

MatchingTerm operator*(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::mul, std::vector<MatchingTerm>({lhs, rhs}),
    MatchingTerm::deduce_term_type(lhs.get_term_type(), rhs.get_term_type()));

  new_term.set_opcode(fheco_trs::OpCode::mul);

  return new_term;
}

MatchingTerm operator+(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::add, std::vector<MatchingTerm>({lhs, rhs}),
    MatchingTerm::deduce_term_type(lhs.get_term_type(), rhs.get_term_type()));
  new_term.set_opcode(fheco_trs::OpCode::add);

  return new_term;
}

MatchingTerm operator-(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::sub, std::vector<MatchingTerm>({lhs, rhs}),
    MatchingTerm::deduce_term_type(lhs.get_term_type(), rhs.get_term_type()));

  new_term.set_opcode(fheco_trs::OpCode::sub);

  return new_term;
}

MatchingTerm operator!(const MatchingTerm &operand)
{
  MatchingTerm new_term(TermType::boolean);
  new_term.set_opcode(OpCode::_not);
  new_term.set_operands({operand});
  return new_term;
}

MatchingTerm operator-(const MatchingTerm &term)
{
  MatchingTerm new_term(fheco_trs::OpCode::negate, std::vector<MatchingTerm>({term}), term.get_term_type());
  return new_term;
}

/*
  These are only for ciphertexts
*/
MatchingTerm exponentiate(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (rhs.get_term_type() != fheco_trs::TermType::rawData)
    throw("invalid exponentiate expression, exponent must of type rawDataType");

  MatchingTerm new_term(
    fheco_trs::OpCode::exponentiate, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::ciphertext);
  return new_term;
}

MatchingTerm square(const MatchingTerm &term)
{
  MatchingTerm new_term(fheco_trs::OpCode::square, std::vector<MatchingTerm>({term}), fheco_trs::TermType::ciphertext);
  return new_term;
}

MatchingTerm relin(const MatchingTerm &term)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::relinearize, std::vector<MatchingTerm>({term}), fheco_trs::TermType::ciphertext);
  return new_term;
}

MatchingTerm mod_switch(const MatchingTerm &term)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::modswitch, std::vector<MatchingTerm>({term}), fheco_trs::TermType::ciphertext);
  return new_term;
}

MatchingTerm negate(const MatchingTerm &term)
{
  MatchingTerm new_term(fheco_trs::OpCode::negate, std::vector<MatchingTerm>({term}), fheco_trs::TermType::ciphertext);
  return new_term;
}

MatchingTerm operator<<(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(
    fheco_trs::OpCode::rotate, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::ciphertext);
  return new_term;
}

/*
  Operators for rewrite condition
*/

MatchingTerm operator!=(const MatchingTerm &lhs, const MatchingTerm &rhs)
{

  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(
    fheco_trs::OpCode::not_equal, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator&&(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(fheco_trs::OpCode::_and, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator<(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(
    fheco_trs::OpCode::less_than, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator<=(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(
    fheco_trs::OpCode::less_than_or_equal, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator>(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(
    fheco_trs::OpCode::greater_than, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator>=(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(
    fheco_trs::OpCode::greater_than_or_equal, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator||(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  MatchingTerm new_term(fheco_trs::OpCode::_or, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

MatchingTerm operator==(const MatchingTerm &lhs, const MatchingTerm &rhs)
{
  if (
    rewrite_condition_types.find(lhs.get_term_type()) == rewrite_condition_types.end() ||
    rewrite_condition_types.find(rhs.get_term_type()) == rewrite_condition_types.end())
  {
    throw("impossible to evaluate rewrite condition");
  }

  MatchingTerm new_term(fheco_trs::OpCode::equal, std::vector<MatchingTerm>({lhs, rhs}), fheco_trs::TermType::boolean);
  return new_term;
}

void MatchingTerm::push_operand(const MatchingTerm &operand)
{
  operands.push_back(operand);
}

/*
  util functions
*/

/*
  for checking condition you create a new MatchingTerm and push input as operand, however for substitution we can just
  set the function id
*/

MatchingTerm MatchingTerm::fold(MatchingTerm term_to_fold)
{
  if (term_to_fold.get_term_type() == fheco_trs::TermType::ciphertext)
    throw("cannot fold ciphertexts");

  term_to_fold.set_function_id(FunctionId::fold);
  return term_to_fold;
}

MatchingTerm MatchingTerm::opcode_of(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::scalar);
  new_term.set_function_id(FunctionId::get_opcode);
  new_term.push_operand(m_term);
  return new_term;
}

MatchingTerm MatchingTerm::isconst(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::boolean);
  new_term.set_function_id(FunctionId::isconst);
  new_term.push_operand(m_term);
  return new_term;
}

MatchingTerm MatchingTerm::depth_of(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::scalar);
  new_term.set_function_id(FunctionId::depth);
  new_term.push_operand(m_term);
  return new_term;
}

MatchingTerm MatchingTerm::iszero(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::boolean);
  new_term.set_function_id(FunctionId::iszero);
  new_term.push_operand(m_term);
  return new_term;
}
MatchingTerm MatchingTerm::isone(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::boolean);
  new_term.set_function_id(FunctionId::isone);
  new_term.push_operand(m_term);
  return new_term;
}

MatchingTerm MatchingTerm::type_of(const MatchingTerm &m_term)
{
  MatchingTerm new_term(TermType::scalar);
  new_term.set_function_id(FunctionId::type_of);
  new_term.push_operand(m_term);
  return new_term;
}
} // namespace fheco_trs
