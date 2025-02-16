import bpy
import logging
from pathlib import Path

class BRTRI_ErrorHandler:
    logger = None
    
    @classmethod
    def initialize(cls):
        cls.logger = logging.getLogger('BRTRI')
        cls.logger.setLevel(logging.INFO)
        
        log_file = Path(__file__).parent.parent / "brtri.log"
        handler = logging.FileHandler(log_file)
        formatter = logging.Formatter('%(asctime)s - %(levelname)s - %(message)s')
        handler.setFormatter(formatter)
        cls.logger.addHandler(handler)
    
    @classmethod
    def log_error(cls, message, context=None):
        if cls.logger:
            if context:
                message = f"{context} - {message}"
            cls.logger.error(message)
        
        # Update UI error message
        bpy.context.scene.brtri_error_message = message
    
    @classmethod
    def cleanup(cls):
        if cls.logger:
            for handler in cls.logger.handlers[:]:
                handler.close()
                cls.logger.removeHandler(handler)
