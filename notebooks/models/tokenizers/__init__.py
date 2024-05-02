from nltk.stem import WordNetLemmatizer
from nltk.tokenize import word_tokenize, sent_tokenize
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

class CustomTokenizer:
    def __init__(self, type="tiktoken"):
        """
        @param type: available types are:
            - tiktoken
            - word
            - sentence
            - byte
            Supported types of tokenizer will also include:
                - lemma
        """
        self.type = type
        match type:
            case "tiktoken":
                self.encoder = CustomEncoder(encoder_decoder=tiktoken.encoding_for_model("gpt-4"))
            case "word":
                self.encoder = CustomEncoder(encode=word_tokenize)
            case "sentence":
                self.encoder = CustomEncoder(encode=sent_tokenize)
            case "byte":
                self.encoder = CustomEncoder(encode=(lambda text: 
                                                       [str(token) for token in (list(map(int, text.encode("utf-8"))))]),
                                             decode=(lambda encoded: "".join([chr(int(token)) for token in encoded])))

    def encode(self, content):
        return self.encoder.encode(content)

    def decode(self, content):
        if self.encoder.encoder_decoder is None and self.encoder.decodeFn is None:
            raise AssertionError("Decode method can't be used for tokenizer of type " + self.type)
        return self.encoder.decode(content)

