<template>
  <div class="logo-panel-container" :class="{ 'expanded': isExpanded }">
    <div class="logo-panel" @click="toggleExpansion">
      <img :src="logoSrc" alt="Logo" class="logo" />
    </div>
    <div class="horizontal-bar-wrapper" v-if="isExpanded">
      <div class="horizontal-bar">
        <Panel class="Faq" label="Faq">
          <p>
          <p>Your account</p>

          <p>How do I change my account email address?</p>
          <p>
            Within…, it’s not possible to change your email address. If you
            want to link a different email address to your account, you will
            have to create a whole new account.
          </p>

          <p>How do I get a new password?</p>
          <p>
            Tap "forgot password" and insert the email address linked to your
            account. Follow the link sent to that email and choose a new
            password.
          </p>

          <p>Use</p>

          <p>Is this every study place in Brussels?</p>
          <p>
            No, unfortunately, our list does not include every study place in
            Brussels, but we are actively adding places to our website.
          </p>

          <p>Is this site only for students?</p>
          <p>
            No, of course not! Everyone is free to use our website and find
            study places. However, only students are able to create an account
            within the website and create a favorite list.
          </p>
          </p>
        </Panel>
        <Panel class="Privacy" label="Privacy">
          <p>
          <p>
            This Privacy Statement explains how we collect, use, disclose,
            and safeguard your personal information when you use our services.
            We are committed to protecting your privacy and ensuring the
            security of your personal information.
          </p>

          <h2>1. Information We Collect</h2>

          <p>
            We may collect personal information from you when you create a
            personal account. The types of personal information we may collect
            include:
          </p>

          <ul>
            <li>Contact information (such as name, email address)</li>
            <li>Login credentials (such as username, password)</li>
          </ul>

          <h2>2. How We Use Your Information</h2>

          <p>
            We use the collected information for the following purposes:
          </p>

          <ul>
            <li>Provide and maintain our services</li>
            <li>Process and fulfill your requests</li>
            <li>Personalize your experience with our services</li>
            <li>Improve our services and develop new features</li>
            <li>Communicate with you, including customer support</li>
            <li>Ensure the security and integrity of our services</li>
          </ul>

          <h2>3. Information Sharing and Disclosure</h2>

          <p>
            We may share your personal information in the following
            circumstances:
          </p>

          <ul>
            <li>With your consent or at your direction</li>
            <li>
              With trusted third-party service providers who assist us in
              operating our services and conducting our business
            </li>
            <li>
              In response to a legal request, such as a court order or
              government investigation
            </li>
            <li>
              To protect our rights, property, or safety, as well as the
              rights, property, or safety of others
            </li>
          </ul>

          <h2>4. Data Security</h2>

          <p>
            We implement appropriate security measures to protect your personal
            information from unauthorized access, use, disclosure, or alteration.
            However, please note that no method of transmission over the internet
            or electronic storage is 100% secure, and we cannot guarantee absolute
            security.
          </p>

          <h2>5. Your Rights and Choices</h2>

          <p>
            You have certain rights and choices regarding the personal information
            we collect and process. These may include:
          </p>

          <ul>
            <li>
              Access and rectification: You can request access to your personal
              information and ask us to correct any inaccuracies.
            </li>
            <li>
              Data deletion: You can request the deletion of your personal
              information, subject to certain legal limitations and requirements.
            </li>
          </ul>

          <h2>6. Children's Privacy</h2>

          <p>
            Our services are not directed to individuals under the age of 13
            (or a higher age if required by applicable law). We do not knowingly
            collect or solicit personal information from children. If we become
            aware that we have collected personal information from a child without
            parental consent, we will take steps to remove that information from
            our records.
          </p>

          <h2>7. Updates to this Privacy Statement</h2>

          <p>
            We may update this Privacy Statement from time to time. We will notify
            you of any material changes by posting the updated Privacy Statement on
            our website or through other communication channels.
          </p>

          <h2>8. Contact Us</h2>

          <p>
            If you have any questions, concerns, or requests regarding this Privacy
            Statement or our privacy practices, please contact us using the following
            information:
          </p>

          <p>StudyHub BXL</p>
          <p>studyhubbxl@contact.com</p>
          </p>
        </Panel>

        <Panel class="Login" v-if="!isSignUpVisible && !isUserLoggedin && !isForgot" label="Login">
          <br>
          <br />
          <h1>Login</h1>
          <hr class="black-bar" />

          <Textfield id="email" label="Mail" placeholder="Enter your email" />
          <br />
          <Textfield id="password" label="Password" placeholder="Enter your password" />
          <br />
          <Checkbox id="keepmeloggedin" label="Keep me logged in!" />
          <br />
          <Button label="LOGIN IN" @click="handleLogin" />
          <br />
          <a @click="toggleForgot" href="#">Forgot password?</a>
          <br />
          <br />
          <br />
          <br />
          <p>Not signed up yet?</p>
          <Button label="Sign Up!" @click="toggleSignUp" />
        </Panel>
        <Panel class="Signup" v-else-if="isSignUpVisible && !isUserLoggedin" label="Sign Up">
          <br />
          <h1>Sign Up</h1>
          <hr class="black-bar" />


          <Textfield id="email-signin" label="Email" placeholder="Enter your email" />
          <br />
          <Textfield id="password-signin" label="Password" placeholder="Enter your password" type="password" />
          <br />
          <Textfield id="confirm-password" label="Confirm Password" placeholder="Confirm Password" type="password" />
          <br />
          <Button label="Sign in" v-if="isSignUpVisible" @click="handleSignUp" />
          <br />
          <p>Already have an account? <a href="#" @click="toggleSignUp">Sign in</a></p>
        </Panel>
        <Panel class="Account" label="Account" v-else-if="isUserLoggedin">
          <h1>Account</h1>
          <hr class="black-bar" />
          <Button id="change-password" label="Change Password" />
          <br>
          <Button id="deleteaccount" label="Delete Account" />
          <br>
          <Button id="logout" label="Logout" @click="toggleLoggedIn" />
        </Panel>
        <Panel class="ForgotPassword" label="Forgot Password" v-if="isForgot">
          <br />
          <Textfield id="emailforgotpassword" label="email" />
          <Button label="Reset Password" @click="handleForgot" />
        </Panel>
      </div>
    </div>
  </div>
</template>

<style scoped>
.Faq {
  opacity: 2;
  width: 33%;
}

.Faq.expanded {
  opacity: 2;
  width: 450px;
}

.Privacy {
  opacity: 2;
  width: 33%;
}

.Privacy.expanded {
  opacity: 2;
  width: 450px;
}

.Login {
  opacity: 2;
  width: 33%;
}

.Login.expanded {
  opacity: 2;
  display: flex;

  width: 450px;
}

.Signup {
  opacity: 2;
  width: 33%;
}

.Signup.expanded {
  opacity: 2;
  width: 450px;
}

.Account {
  opacity: 2;
  width: 33%;
}

.Account.expanded {
  opacity: 2;
  width: 450px;
}

.ForgotPassword {
  opacity: 2;
  width: 33%;
}

.ForgotPassword.expanded {
  opacity: 2;
  width: 450px;

}

.logo-panel-container {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  height: 200px;
  transition: transform 0.3s ease-in-out;
}

.black-bar {
  border: none;
  height: 2px;
  background-color: black;
}

.expanded {
  transform: translateX(-0%);
}

.logo-panel {
  border: 1px solid black;
  width: 100px;
  height: 100px;
  border-radius: 50%;
  background-color: #2F86A6;
  display: flex;
  justify-content: center;
  align-items: center;
  border-width: 1px;
  border-color: black;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
  cursor: pointer;
}

.logo {
  width: 80px;
  height: 80px;
}

.horizontal-bar-wrapper {
  position: relative;
  margin-left: 10px;
}

.horizontal-bar {
  border-radius: 8px;
  display: flex;
  border: 2px solid black;
  width: 600px;
  height: 40px;
  background-color: #2F86A6;

}
</style>

<script lang="ts">
import { post } from "@/fetch";
import Panel from './Panel.vue';
import Checkbox from './Checkbox.vue';
import Textfield from './Textfield.vue';
import Button from './Button.vue';

export default {
  props: {
    logoSrc: {
      type: String,
      default: '/logo.png'
    }
  },
  data() {
    return {
      isExpanded: false,
      isSignUpVisible: false,
      isUserLoggedin: false,
      isForgot: false,
    };
  },
  methods: {
    async handleForgot() {
      let emailforgotpassword = document.getElementById("emailforgotpassword").value;
      
      this.toggleForgot();
      let result = await post<Object, String>("http://localhost:8080/api/v1/users/auth/forgotpassword", { 
        "email": emailforgotpassword,
      });
    },

   async handleLogin() {
      let email = document.getElementById("email").value;
      let password = document.getElementById("password").value;
      let keepmeloggedin = document.getElementById("keepmeloggedin").checked;




      let result = await post<Object, String>("http://localhost:8080/api/v1/users/auth/login", { 
        "email": email, "password": password
      });
    },

    async handleSignUp() {
      let emailsignin = document.getElementById("email-signin").value;
      let passwordsignin = document.getElementById("password-signin").value;
      let confirmpassword = document.getElementById("confirm-password").value;
      if(passwordsignin!=confirmpassword){
        return(console.error("Passwords not matching"));
        

      }
      let result = await post<Object, String>("http://localhost:8080/api/v1/users/auth/register", { 
        "email": emailsignin, "password": passwordsignin
      });
    },

    toggleExpansion() {
      this.isExpanded = !this.isExpanded;
    },
    toggleSignUp() {
      this.isSignUpVisible = !this.isSignUpVisible;
    },
    toggleLoggedIn() {
      this.isUserLoggedin = !this.isUserLoggedin;
    },
    toggleForgot() {
      this.isForgot = !this.isForgot;
    }
  },

  components: {
    Panel,
    Checkbox,
    Textfield,
    Button,
  },
};
</script>
