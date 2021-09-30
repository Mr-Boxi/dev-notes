## agent模块  
xuperos/agent  
代理依赖组件    

目录：  
    引擎依赖代理组件 EngineRelyAgentImpl   
    链依赖代理组件  ChainRelyAgentImpl  
    链核心代理      ChainCoreAgent     
    账本代理       LedgerAgent  

**type EngineRelyAgentImpl struct**  //引擎代理依赖组件实例化操作   
```
type EngineRelyAgentImpl struct {
    engine common.Engine
} 
//对引擎进行接口进行封装一层
```
* func NewEngineRelyAgent(engine common.Engine) *EngineRelyAgentImpl   
* func (t *EngineRelyAgentImpl) CreateNetwork(envCfg *xconf.EnvConf) (network.Network, error) //创建并启动p2p网络     

**type ChainRelyAgentImpl struct**   //代理依赖组件实例化操作，方便mock单测和并行开发  
```
type ChainRelyAgentImpl struct {
    chain common.Chain
}  
// 本质是对链接口进行一层封装
``` 
* func NewChainRelyAgent(chain common.Chain) *ChainRelyAgentImpl   
* func (t *ChainRelyAgentImpl) CreateLedger() (*ledger.Ledger, error) //创建账本  
* func (t *ChainRelyAgentImpl) CreateState(leg *ledger.Ledger, crypt cryptoBase.CryptoClient) (*state.State, error) // 创建状态机  
* func (t *ChainRelyAgentImpl) CreateCrypto(cryptoType string) (cryptoBase.CryptoClient, error)   
* func (t *ChainRelyAgentImpl) CreateAcl() (aclBase.AclManager, error)   
* func (t *ChainRelyAgentImpl) CreateContract(xmreader kledger.XMReader) (contract.Manager, error)  
* func (t *ChainRelyAgentImpl) CreateConsensus() (consensus.ConsensusInterface, error)   
* func (t *ChainRelyAgentImpl) CreateGovernToken() (governToken.GovManager, error)  
* func (t *ChainRelyAgentImpl) CreateProposal() (propose.ProposeManager, error) 
* func (t *ChainRelyAgentImpl) CreateTimerTask() (timerTask.TimerManager, error) 

**type ChainCoreAgent struct**  //链代理
``` 
type ChainCoreAgent struct {
    log      logs.Logger
    chainCtx *common.ChainCtx  //链上下文
}
// 本质是对链上下文进行封装
// 链的上下文结构在commond模块中
```
* func NewChainCoreAgent(chainCtx *common.ChainCtx) *ChainCoreAgent    
* func (t *ChainCoreAgent) GetAccountAddresses(accountName string) ([]string, error) 
  // 查询合约acl      
* func (t *ChainCoreAgent) VerifyContractPermission(initiator string, authRequire []string, contractName, methodName string) (bool, error)  //结合合约acl设置鉴权  
* func (t *ChainCoreAgent) VerifyContractOwnerPermission(contractName string, authRequire []string) error  // 结合合约acl设置鉴权  
* func (t *ChainCoreAgent) QueryTransaction(txid []byte) (*pb.Transaction, error) // 查询交易    
* func (t *ChainCoreAgent) QueryBlock(blockid []byte) (ledger.BlockHandle, error)  //查询区块  

**type LedgerAgent struct**  //账本代理
``` 
type LedgerAgent struct {
    log      logs.Logger
    chainCtx *common.ChainCtx
}
// 本质是对链上下文进行封装
```
* func NewLedgerAgent(chainCtx *common.ChainCtx) *LedgerAgent   func (t *LedgerAgent) GetNewAccountGas() (int64, error)   
* func (t *LedgerAgent) GetNewGovGas() (int64, error)   //从创世块获取治理代币消耗gas  
* func (t *LedgerAgent) GetGenesisPreDistribution() ([]ledger.Predistribution, error)  
* func (t *LedgerAgent) GetCryptoType() (string, error)  
* func (t *LedgerAgent) GetConsensusConf() ([]byte, error)   //从创世块获取共识配置  
* func (t *LedgerAgent) QueryBlock(blkId []byte) (kledger.BlockHandle, error)  
* func (t *LedgerAgent) QueryBlockByHeight(height int64) (kledger.BlockHandle, error)   
* func (t *LedgerAgent) GetTipBlock() kledger.BlockHandle  
* func (t *LedgerAgent) GetTipXMSnapshotReader() (kledger.XMSnapshotReader, error)   
* func (t *LedgerAgent) CreateSnapshot(blkId []byte) (kledger.XMReader, error)   
* func (t *LedgerAgent) GetTipSnapshot() (kledger.XMReader, error)  
* func (t *LedgerAgent) CreateXMReader() kledger.XMReader   
   
 