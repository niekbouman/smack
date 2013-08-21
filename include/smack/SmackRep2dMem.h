//
// Copyright (c) 2013 Zvonimir Rakamaric (zvonimir@cs.utah.edu),
//                    Michael Emmi (michael.emmi@gmail.com)
// This file is distributed under the MIT License. See LICENSE for details.
//
#ifndef SMACKREP2DMEM_H
#define SMACKREP2DMEM_H

#include "smack/SmackRep.h"

namespace smack {

using llvm::Regex;
using llvm::SmallVector;
using llvm::StringRef;
using namespace std;

class SmackRep2dMem : public SmackRep {
public:
  static const string PTR_TYPE;
  static const string REF_TYPE;
  static const string POINTERS;

public:
  SmackRep2dMem(llvm::AliasAnalysis* aa) : SmackRep(aa) {}
  virtual vector<const Decl*> globalDecl(const llvm::Value* g);
  virtual vector<string> getModifies();
  virtual string getPtrType();
  
  const Expr* ptr2val(const Expr* e);
  const Expr* val2ptr(const Expr* e);
  const Expr* ref2ptr(const Expr* e);
  
  virtual string memoryModel();
  virtual string mallocProc();
  virtual string freeProc();
  virtual string allocaProc();
  virtual string memcpyProc(int dstReg, int srcReg);
};
}

#endif // SMACKREP2DMEM_H

