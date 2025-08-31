import sys
import json
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('all-MiniLM-L6-v2')
texts = json.loads(sys.argv[1])
embeddings = model.encode(texts)
print(json.dumps(embeddings.tolist()))
