import logging
import bpy

class DebugUtils:
    logger = None

    @classmethod
    def initialize(cls):
        cls.logger = logging.getLogger('BRTRI_Debug')
        cls.logger.setLevel(logging.DEBUG)
        
        handler = logging.StreamHandler()
        formatter = logging.Formatter('%(asctime)s - %(levelname)s - %(message)s')
        handler.setFormatter(formatter)
        cls.logger.addHandler(handler)

    @classmethod
    def log_debug(cls, message):
        if cls.logger:
            cls.logger.debug(message)
        
        # Update UI debug message
        bpy.context.scene.brtri_debug_message = message

    @classmethod
    def log_info(cls, message):
        if cls.logger:
            cls.logger.info(message)
        
        # Update UI info message
        bpy.context.scene.brtri_info_message = message

    @classmethod
    def log_warning(cls, message):
        if cls.logger:
            cls.logger.warning(message)
        
        # Update UI warning message
        bpy.context.scene.brtri_warning_message = message

    @classmethod
    def log_error(cls, message):
        if cls.logger:
            cls.logger.error(message)
        
        # Update UI error message
        bpy.context.scene.brtri_error_message = message

    @classmethod
    def log_data_processing_step(cls, step, data):
        if cls.logger:
            cls.logger.debug(f"Step: {step}, Data: {data}")

    @classmethod
    def cleanup(cls):
        if cls.logger:
            for handler in cls.logger.handlers[:]:
                handler.close()
                cls.logger.removeHandler(handler)
