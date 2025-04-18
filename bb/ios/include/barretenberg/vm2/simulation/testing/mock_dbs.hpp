#include "barretenberg/vm2/simulation/lib/db_interfaces.hpp"

#include <cassert>
#include <gmock/gmock.h>
#include <optional>

namespace bb::avm2::simulation {

class MockContractDB : public ContractDBInterface {
    // https://google.github.io/googletest/gmock_cook_book.html#making-the-compilation-faster
    MockContractDB();
    ~MockContractDB() override;

    MOCK_METHOD(std::optional<ContractInstance>,
                get_contract_instance,
                (const AztecAddress& address),
                (const, override));
    MOCK_METHOD(std::optional<ContractClass>, get_contract_class, (const ContractClassId& class_id), (const, override));
};

class MockMerkleDB : public MerkleDBInterface {
    // https://google.github.io/googletest/gmock_cook_book.html#making-the-compilation-faster
    MockMerkleDB();
    ~MockMerkleDB() override;

    MOCK_METHOD(const TreeSnapshots&, get_tree_roots, (), (const, override));
};

} // namespace bb::avm2::simulation
