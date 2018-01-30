//
// This file is distributed under the MIT License. See LICENSE for details.
//
#ifndef INTEGERPACKINGTOSTRUCT_H
#define INTEGERPACKINGTOSTRUCT_H

#include "llvm/Pass.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Instructions.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/Type.h"
#include "llvm/IR/IntrinsicInst.h"
#include <vector>
#include <utility>

namespace smack {

typedef std::pair<llvm::Type*, unsigned> TypeInfoT;

class IntegerPackingToStruct: public llvm::ModulePass {
public:
  static char ID;
  IntegerPackingToStruct() : llvm::ModulePass(ID) {}
  const char* getPassName() const;
  virtual bool runOnModule(llvm::Module& M);
private:
  const llvm::DataLayout* DL;
  llvm::Value* getBasePointer(llvm::Value* ptr);
  llvm::AllocaInst* getBox(llvm::Value* ptr);
  bool isBox(llvm::Value* ptr);
  TypeInfoT makeTypeInfo(llvm::Type* T, unsigned offset);
  std::vector<TypeInfoT> flattenType(llvm::Type* T, unsigned offset);
  bool isTypeEqual(llvm::Type* EA, llvm::Type* EB, unsigned size);
  void detectPackingInLoad(llvm::LoadInst* li);
  void detectPackingInMemCpy(llvm::MemCpyInst* mci);
  void detectPacking(llvm::Function* F);
};
}

#endif
