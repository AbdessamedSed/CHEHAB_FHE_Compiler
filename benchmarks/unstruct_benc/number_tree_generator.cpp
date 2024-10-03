using namespace std;
#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <sstream>
#include <random>
#include <fstream>

class Tree {
public:
    virtual ~Tree() = default;
    virtual void print(ostream& out) const = 0;
    virtual string extractExpression() const = 0;
};

class Val : public Tree {
private:
    string value;

public:
    explicit Val(const string& val) : value(val) {}

    void print(ostream& out) const override {
        out << "Ciphertext(\"v_" << value << "\")";
    }

    string extractExpression() const override {
        return "Ciphertext(\"v_" + value + "\")";
    }
};

class Op : public Tree {
private:
    unique_ptr<Tree> lhs, rhs;
    char op;

public:
    Op(unique_ptr<Tree> left, unique_ptr<Tree> right, char operation)
        : lhs(move(left)), rhs(move(right)), op(operation) {}

    void print(ostream& out) const override {
        out << "(";
        lhs->print(out);
        out << " " << op << " ";
        rhs->print(out);
        out << ")";
    }

    string extractExpression() const override {
        string left_expr = lhs->extractExpression();
        string right_expr = rhs->extractExpression();
        return "(" + left_expr + " " + op + " " + right_expr + ")";
    }
};

class Var : public Tree {
private:
    string value;

public:
    explicit Var(const string& val) : value(val) {}

    void print(ostream& out) const override {
        out << "Ciphertext(\"v_" << value << "\")";
    }

    string extractExpression() const override {
        return "Ciphertext(\"v_" + value + "\")";
    }
};

unique_ptr<Tree> treeGenerator(int originalDepth, int maxDepth, int& seed, const string& regime) {
    static mt19937 rng(seed);
    uniform_int_distribution<int> dist0_1(0, 1);
    uniform_int_distribution<int> dist0_3(0, 3);
    uniform_int_distribution<int> dist0_1023(0, 1023);

    if (originalDepth == maxDepth || maxDepth == originalDepth - 1) {
        int randomNum = (regime == "100-100") ? dist0_1(rng) : dist0_1(rng);
        auto lhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);
        auto rhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);

        if (randomNum == 1) {
            return make_unique<Op>(move(lhs), move(rhs), '+');
        } else {
            return make_unique<Op>(move(lhs), move(rhs), '*');
        }
    }

    if (maxDepth > 0) {
        int randomNum = (regime == "50-50") ? dist0_3(rng) :
                        (regime == "100-50") ? dist0_1(rng) :
                        0;
        ++seed;

        if (randomNum > 1) {
            string localString = to_string(dist0_1023(rng));
            ++seed;
            return make_unique<Var>(localString);
        } else {
            auto lhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);
            auto rhs = treeGenerator(originalDepth, maxDepth - 1, seed, regime);

            if (randomNum == 1) {
                return make_unique<Op>(move(lhs), move(rhs), '+');
            } else {
                return make_unique<Op>(move(lhs), move(rhs), '*');
            }
        }
    } else {
        string endNode = to_string(dist0_1023(rng));
        ++seed;
        return make_unique<Var>(endNode);
    }
}

void tree_generator(int originalDepth, int maxDepth, int& seed, const string& regime) {
    // Open the file in ofstream mode, which clears the file if it already exists
    ofstream file("generated_expression.txt");

    if (!file) {
        cerr << "Error: Could not open file." << endl;
        return;
    }

    auto tree = treeGenerator(originalDepth, maxDepth, seed, regime);


    // Extract the expression and write it to the file
    string expression = tree->extractExpression();
    file << expression << endl;

    file.close();
}


int main() {
    int seed = 42;
    string regime = "100-100"; /* 100-100 , 50-50 , 100-50*/
    int originalDepth = 10;
    int maxDepth = 10;

    tree_generator(originalDepth, maxDepth, seed, regime);

    return 0;
}
