//
// This file is distributed under the MIT License. See LICENSE for details.
//

//
// This pass converts LLVM's checked integer-arithmetic operations into basic
// operations, and optionally allows for the checking of overflow.
//

#include "smack/IntegerPackingToStruct.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/InstIterator.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/InstIterator.h"
#include "smack/Debug.h"
#include <algorithm>
#include <utility>

namespace smack {

using namespace llvm;

Value* IntegerPackingToStruct::getBasePointer(Value* ptr) {
  return ptr->stripPointerCasts();
}

AllocaInst* IntegerPackingToStruct::getBox(Value* ptr) {
  return cast<AllocaInst>(ptr);
}

bool IntegerPackingToStruct::isBox(Value* ptr) {
  return isa<AllocaInst>(ptr);
}

TypeInfoT IntegerPackingToStruct::makeTypeInfo(Type* T, unsigned offset) {
  return std::make_pair(T, offset);
}

std::vector<TypeInfoT> IntegerPackingToStruct::flattenType(Type* T, unsigned offset) {
  if (T->isIntegerTy() || T->isFloatingPointTy() || T->isPointerTy())
    return std::vector<TypeInfoT>{makeTypeInfo(T, offset)};
  else if (T->isArrayTy()) {
    ArrayType* st = cast<ArrayType>(T);
    std::vector<TypeInfoT> ret;
    unsigned elemSize = DL->getTypeAllocSize(st->getElementType());
    for (unsigned i = 0, newOffset = offset; i < st->getNumElements(); i += 1, newOffset += elemSize) {
      std::vector<TypeInfoT> sub = flattenType(st->getElementType(), newOffset);
      ret.insert(ret.end(), sub.begin(), sub.end());
    }
    return ret;
  }
  else if (T->isStructTy()) {
    std::vector<TypeInfoT> ret;
    const StructLayout* sl = DL->getStructLayout(cast<StructType>(T));
    for (unsigned i = 0; i < T->getStructNumElements(); i += 1) {
      unsigned newOffset = sl->getElementOffset(i);
      std::vector<TypeInfoT> sub = flattenType(T->getStructElementType(i), offset+newOffset);
      ret.insert(ret.end(), sub.begin(), sub.end());
    }
    return ret;
  } else
    llvm_unreachable("Unsupported type");
}

bool IntegerPackingToStruct::isTypeEqual(Type* EA, Type* EB, unsigned size) {
  std::vector<TypeInfoT> fta = flattenType(EA, 0);
  std::vector<TypeInfoT> ftb = flattenType(EB, 0);
  unsigned ts = std::min(fta.size(), ftb.size());
  unsigned as = 0;
  for (unsigned i = 0; i < size && i < ts && as < size; ++i) {
    Type* ta = std::get<0>(fta[i]);
    Type* tb = std::get<0>(ftb[i]);
    unsigned oa = std::get<1>(fta[i]);
    unsigned ob = std::get<1>(ftb[i]);
    as = std::max(oa, ob);
    if (ta->isPointerTy() && tb->isPointerTy())
      continue;
    //if (ta->getPrimitiveSizeInBits() != tb->getPrimitiveSizeInBits()
    if (DL->getTypeStoreSize(ta) != DL->getTypeStoreSize(tb)
       || oa != ob)
      return false;
  }
  return true;
}

void IntegerPackingToStruct::detectPackingInLoad(LoadInst* li) {
  if (BitCastInst* bi = dyn_cast<BitCastInst>(li->getPointerOperand())) {
    Type* EA = bi->getSrcTy()->getPointerElementType();
    Type* EB = bi->getDestTy()->getPointerElementType();
    if (isBox(getBasePointer(bi->getOperand(0)))
        && !isTypeEqual(EA, EB,std::min(DL->getTypeStoreSize(EA), DL->getTypeStoreSize(EB))))
      errs() << "Got one load: " << " in " << li->getFunction()->getName() << "\n";
  }
}

void IntegerPackingToStruct::detectPackingInMemCpy(MemCpyInst* mci) {
  Value* dest = getBasePointer(mci->getArgOperand(0));
  Value* src = getBasePointer(mci->getArgOperand(1));
  if (isBox(dest) && isBox(src) && isa<ConstantInt>(mci->getLength())) {
    unsigned size = cast<ConstantInt>(mci->getLength())->getZExtValue();
    Type* DT = dest->getType()->getPointerElementType();
    Type* ST = src->getType()->getPointerElementType();
    if (!isTypeEqual(DT, ST, size))
      errs() << "Got one memcpy: " << "in " << mci->getFunction()->getName() << "\n";
  }
}

void IntegerPackingToStruct::detectPacking(Function* F) {
  for (inst_iterator I = inst_begin(F), E = inst_end(F); I != E; ++I) {
    if (LoadInst* li = dyn_cast<LoadInst>(&*I))
      detectPackingInLoad(li);
    else if (MemCpyInst* mci = dyn_cast<MemCpyInst>(&*I))
      detectPackingInMemCpy(mci);
  }
}

bool IntegerPackingToStruct::runOnModule(Module& M) {
  DL = &M.getDataLayout();
  for (Function& f : M) {
    detectPacking(&f);
  }
  return true;
}

char IntegerPackingToStruct::ID = 0;

const char* IntegerPackingToStruct::getPassName() const {
  return "Find Integer Packing";
}
}
