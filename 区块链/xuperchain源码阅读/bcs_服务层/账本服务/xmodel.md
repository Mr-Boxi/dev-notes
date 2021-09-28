### xmodel
xumodel 模型

**type XModel struct**
```
// XModel xmodel data structure
type XModel struct {
	ledger          *ledger.Ledger
	stateDB         kvdb.Database
	unconfirmTable  kvdb.Database
	extUtxoTable    kvdb.Database
	extUtxoDelTable kvdb.Database
	logger          logs.Logger
	batchCache      *sync.Map
	lastBatch       kvdb.Batch
	// extUtxoCache caches per bucket key-values using version as key
	extUtxoCache sync.Map // map[string]*LRUCache
} 
```

