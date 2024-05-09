from nltk.stem import WordNetLemmatizer
from nltk.tokenize import word_tokenize, sent_tokenize
from spacy.lang.en.stop_words import STOP_WORDS as en_stop
import tiktoken

class CustomEncoder:
    def __init__(self, encode=None, decode=None, encoder_decoder=None):
        self.encodeFn = encode
        self.decodeFn = decode
        self.encoder_decoder = encoder_decoder

    def encode(self, content):
        if self.encoder_decoder is not None:
            return self.encoder_decoder.encode(content)
        return self.encodeFn(content)

    def decode(self, content):
        if self.encoder_decoder is not None:
            return self.encoder_decoder.decode(content)
        return self.decodeFn(content)

def __lemmatize(text):
    wnl = WordNetLemmatizer()
    return [wnl.lemmatize(t) for t in word_tokenize(text)]

class CustomTokenizer:
    def __init__(self, type="tiktoken", remove_stopwords=False, to_lower=False):
        """
        @param type: available types are:
            - tiktoken
            - word
            - sentence
            - byte
            - lemma
        """
        self.type = type
        self.remove_stopwords = remove_stopwords
        self.to_lower = to_lower
        if type == "tiktoken":
            self.encoder = CustomEncoder(encoder_decoder=tiktoken.encoding_for_model("gpt-4"))
        elif type == "word":
            self.encoder = CustomEncoder(encode=word_tokenize)
        elif type == "lemma":
            self.encoder = CustomEncoder(encode=__lemmatize)
        elif type == "sentence":
            self.encoder = CustomEncoder(encode=sent_tokenize)
        elif type == "byte":
            self.encoder = CustomEncoder(encode=(lambda text: 
                                                 [str(token) for token in (list(map(int, text.encode("utf-8"))))]),
                                         decode=(lambda encoded: "".join([chr(int(token)) for token in encoded])))

    def encode(self, content: str):
        if self.to_lower:
            content = content.lower()

        if self.remove_stopwords:
            content = " ".join([word for word in content.lower().split() if word not in en_stop])

        return self.encoder.encode(content if not self.to_lower else content.lower())
        

    def decode(self, content):
        if self.encoder.encoder_decoder is None and self.encoder.decodeFn is None:
            raise AssertionError("Decode method can't be used for tokenizer of type " + self.type)
        return self.encoder.decode(content)

