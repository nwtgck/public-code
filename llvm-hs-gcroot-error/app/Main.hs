-- Use llvm.gcroot

{-# LANGUAGE OverloadedStrings #-}

module Main where

import qualified LLVM.AST as AST
import LLVM.AST( Named( (:=) ) )
import qualified LLVM.AST.Global as AST.Global
import qualified LLVM.AST.Constant as AST.Constant
import qualified LLVM.Context as Context
import qualified LLVM.Module as Module
import qualified LLVM.AST.AddrSpace as AST.AddrSpace
import qualified LLVM.AST.CallingConvention as AST.CallingConvention
import qualified LLVM.AST.Type as AST.Type
import qualified LLVM.Target as Target
import qualified LLVM.AST.FunctionAttribute as AST.FunctionAttribute

import qualified Data.ByteString.Char8 as BS
import Data.Char
import Control.Monad.Except


main :: IO ()
main = do
  -- Print LLVM IR (it's fine!)
  putStrLn("============ PRINT LLVM IR ============")
  toLLVM module_
  -- Create object file bytestring (Runtime error...)
  putStrLn("============ GENERATE OBJ FILE BYTESTRING ============")
  objStr <- toObjByteString module_
  return ()

module_ :: AST.Module
module_ = AST.defaultModule
  { AST.moduleName = "basic"
  , AST.moduleDefinitions = [ AST.GlobalDefinition llvmGCRootFuncGlobal
                            , mainDef
                            ]
  }

-- (from: https://github.com/llvm-hs/llvm-hs-examples/blob/238b30306cf3d16a09f95fc5a4199f1ec4a65f8a/basic/Main.hs#L46)
toLLVM :: AST.Module -> IO ()
toLLVM mod = Context.withContext $ \ctx -> do
  llvm <- Module.withModuleFromAST ctx mod Module.moduleLLVMAssembly
  BS.putStrLn llvm

toObjByteString :: AST.Module -> IO BS.ByteString
toObjByteString mod = Context.withContext $ \ctx ->
  Target.withHostTargetMachine $ \target ->
      Module.withModuleFromAST ctx mod (Module.moduleObject target)



-- LL: void @llvm.gcroot(i8** %ptrloc, i8* %metadata)
llvmGCRootFuncGlobal :: AST.Global
llvmGCRootFuncGlobal = AST.functionDefaults
 {  AST.Global.name       = AST.Name "llvm.gcroot"
  , AST.Global.returnType = AST.Type.void
  , AST.Global.parameters = ([ AST.Parameter (AST.Type.ptr (AST.Type.ptr AST.Type.i8)) (AST.Name "ptrloc") []
                             , AST.Parameter (AST.Type.ptr AST.Type.i8) (AST.Name "metadata") []
                             ], False)
  -- (from: @cocreature https://github.com/llvm-hs/llvm-hs/issues/198#issuecomment-379305825)
  , AST.Global.functionAttributes = [Right AST.FunctionAttribute.NoUnwind]
 }

mainDef :: AST.Definition
mainDef = AST.GlobalDefinition AST.Global.functionDefaults
  { AST.Global.name = AST.Name "main"
  , AST.Global.parameters =([], False)
  , AST.Global.returnType = AST.Type.i32
  , AST.Global.basicBlocks = [body]
  , AST.Global.garbageCollectorName = Just "shadow-stack"
  }
  where

    -- LL: void (i8**, i8*)*
    llvmGcRootFuncTy =
      AST.Type.ptr
        AST.FunctionType {
          AST.resultType    = AST.Type.void
        , AST.argumentTypes = [AST.Type.ptr (AST.Type.ptr AST.Type.i8), AST.Type.ptr AST.Type.i8]
        , AST.isVarArg      = False
        }

    body = AST.Global.BasicBlock
        (AST.Name "entry")
        [ -- LL: %my_ptr = alloca i8*
          AST.Name "my_ptr" := AST.Alloca {
             AST.allocatedType = AST.Type.ptr AST.Type.i8,
             AST.numElements   = Nothing,
             AST.alignment     = 0,
             AST.metadata      = []
           }
          -- LL: call void @llvm.gcroot(i8** %my_ptr, i8* null)
        , AST.Do AST.Call {
            AST.tailCallKind       = Nothing,
            AST.callingConvention  = AST.CallingConvention.C,
            AST.returnAttributes   = [],
            AST.function           = Right $ AST.ConstantOperand $ AST.Constant.GlobalReference llvmGcRootFuncTy (AST.Name "llvm.gcroot"),
            AST.arguments          = [ (AST.LocalReference (AST.Type.ptr (AST.Type.ptr AST.Type.i8)) (AST.Name "my_ptr"), [])
                                     , (AST.ConstantOperand AST.Constant.Null { AST.Constant.constantType = AST.Type.ptr AST.Type.i8 }, [])
                                     ],
            AST.functionAttributes = [],
            AST.metadata           = []
          }
        ]
        -- LL: ret i32 0
        (AST.Do AST.Ret {
          AST.returnOperand = Just (AST.ConstantOperand AST.Constant.Int {AST.Constant.integerBits=32, AST.Constant.integerValue=0} ),
          AST.metadata'     = []
        })