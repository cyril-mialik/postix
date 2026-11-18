CREATE TABLE profile_verification_type (
    profile_id UUID NOT NULL,
    verification_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT now(),

    PRIMARY KEY (profile_id, verification_id),
    
    CONSTRAINT fk_profile_verification_type_profile 
        FOREIGN KEY (profile_id) REFERENCES profile(id) 
        ON DELETE CASCADE ON UPDATE CASCADE,
        
    CONSTRAINT fk_profile_verification_type_verification_type 
        FOREIGN KEY (verification_id) REFERENCES verification_type(id) 
        ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_profile_verification_type_id 
    ON profile_verification_type (verification_id);
